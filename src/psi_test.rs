use curve25519_dalek::constants::RISTRETTO_BASEPOINT_TABLE;
use curve25519_dalek::traits::Identity;
use curve25519_dalek::RistrettoPoint;
use curve25519_dalek::Scalar;

use crate::okvs;
use fxhash::hash64;

pub const DIM: usize = 2;
pub const R: u64 = 20; // radius 半径
pub const SIDE_LEN: u64 = 2 * R; // 边长，直径
pub const BLK_CELLS: usize = 1 << DIM; //2^DIM
pub const R_L2: u64 = R * R;
pub type Point = [u64; DIM];

/*
计算一个给定点 p 在一个 d 维空间中根据边长 sidele 所对应的块的左下角坐标 Point
*/
#[inline]
fn cell(p: &Point, sidele: u64) -> Point {
    let mut bot_left_corner: Point = [0u64; DIM];
    for i in 0..DIM {
        bot_left_corner[i] = p[i] / sidele;
    }
    return bot_left_corner;
}

/*
计算一个给定点 p, 根据一个半径 radius 和边长 sidele 所对应的块的左下角坐标 Point
计算边界点所在的块
*/
#[inline]
fn block(p: &Point, sidele: u64, radius: u64) -> Point {
    let mut min: Point = [0u64; DIM];
    for i in 0..DIM {
        // 计算给定点 p 在某个维度 i 上的坐标减去一个半径值
        // 确定一个新的坐标，这个坐标通常用于定义一个区域或块的边界
        min[i] = p[i] - radius;
    }
    return cell(&min, sidele);
}

// 计算两个点 p1 和 p2 之间的 L2 距离（欧几里得距离的平方）
#[inline]
fn l2_dist(p1: &Point, p2: &Point) -> u64 {
    let mut sum: u64 = 0;
    let mut diff: u64;
    for i in 0..DIM {
        // 计算每个维度的差值的绝对值
        diff = (p1[i] as i64 - p2[i] as i64).abs() as u64;
        // 将差值的平方累加到总和
        sum += diff * diff;
    }
    return sum;
}

// 计算每个维度的差值的绝对值，并累加
#[inline]
fn l1_dist(p1: &Point, p2: &Point) -> u64 {
    let mut sum: u64 = 0;
    for i in 0..DIM {
        sum += (p1[i] as i64 - p2[i] as i64).abs() as u64;
    }
    return sum;
}

// 计算给定点 p 相对于源点 source 的位置索引
#[inline]
fn get_position(p: &Point, source: &Point) -> usize {
    let mut pos: usize = 0;
    for i in 0..DIM {
        // 如果 p 的某一维度大于 source 的对应维度，则更新位置索引
        if p[i] > source[i] {
            pos += 1 << i;
        }
    }
    return pos;
}

// 计算两个点之间的无穷范数（切比雪夫距离）
#[inline]
fn l_inf_dist(p1: &Point, p2: &Point) -> u64 {
    let mut max_diff: u64 = 0; // 初始化最大差值
    for i in 0..DIM {
        let diff = (p1[i] as i64 - p2[i] as i64).abs() as u64; // 计算每个维度的差值
        if diff > max_diff {
            max_diff = diff; // 更新最大差值
        }
    }
    return max_diff; // 返回无穷范数
}

#[inline]
fn intersection(p: &Point, metric: u32) -> Vec<Point> {
    // 检查维度是否为2
    if DIM != 2 {
        panic!("DIM should be 2");
    }
    // 初始化结果向量，容量为 BLK_CELLS
    let mut result: Vec<Point> = Vec::with_capacity(BLK_CELLS);
    // 计算给定点 p 所在块的左下角坐标
    let blk = block(p, SIDE_LEN, R);
    // 初始化交叉点
    let mut cross_point: Point = [0u64; DIM];
    // 计算交叉点的坐标
    for j in 0..DIM {
        cross_point[j] = blk[j] * SIDE_LEN + SIDE_LEN;
    }
    let dist;
    // 根据度量计算距离
    if metric == 2 {
        dist = l2_dist(p, &cross_point);
    } else if metric == 1 {
        dist = l1_dist(p, &cross_point);
    } else {
        panic!("metric should be L1 or L2");
    }
    // 获取交叉点相对于源点 p 的位置索引
    let pos_ind = get_position(&cross_point, p);
    // 遍历所有块
    for i in 0..BLK_CELLS {
        let mut tem: Point = [0u64; DIM];
        // 根据度量选择半径
        let r_lp = if metric == 2 { R_L2 } else { R };
        // 如果距离大于半径且当前块是交叉点的位置，则跳过
        if (dist > r_lp) && (i == pos_ind) {
            continue;
        }
        // 计算当前块的坐标
        for j in 0..DIM {
            if (i >> j) & 1 == 1 {
                tem[j] = blk[j] + 1;
            } else {
                tem[j] = blk[j];
            }
        }
        // 将当前块的坐标添加到结果中
        result.push(tem);
    }
    // 返回结果
    return result;
}

pub struct Receiver {
    window: usize,                         // 窗口大小
    pk: RistrettoPoint,                    // 公钥
    sk: Scalar,                            // 私钥
    _pre_data: Vec<Vec<(Scalar, Scalar)>>, // 预处理数据
    _okvsgen: Vec<okvs::OkvsGen>,          // okvs生成器
}

impl Receiver {
    // 1 vs 1 test new
    pub fn new() -> Self {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let sk: Scalar = Scalar::random(&mut rng); // 随机生成私钥
        let pk: RistrettoPoint = &sk * RISTRETTO_BASEPOINT_TABLE; // 根据私钥计算公钥
        let mut _pre_data: Vec<Vec<(Scalar, Scalar)>> = Vec::with_capacity(DIM); // 初始化预处理数据
        let mut _okvsgen: Vec<okvs::OkvsGen> = Vec::with_capacity(DIM); // 初始化 okvs 生成器

        let window = 2 * R + 1; // 否则窗口大小为 (2 * R + 1)

        for _ in 0..DIM {
            let mut pair: Vec<(Scalar, Scalar)> = Vec::with_capacity(window as usize); // 为每一维度初始化一个空的预处理数据
            for _ in 0..window {
                let tem: Scalar = Scalar::random(&mut rng); // 随机生成数据
                pair.push((tem, tem * sk)); // 将数据和数据乘以私钥存入配对中
            }
            _pre_data.push(pair); // 将每一维度的预处理数据加入 _pre_data
            _okvsgen.push(okvs::OkvsGen::new(window)); // 为每一维度初始化 okvs 生成器
        }

        return Receiver {
            window: window as usize,
            pk,
            sk,
            _pre_data,
            _okvsgen,
        };
    }

    // 返回公钥
    pub fn publish_pk(&self) -> RistrettoPoint {
        return self.pk;
    }

    // 返回窗口大小
    pub fn get_windowsize(&self) -> usize {
        return self.window;
    }

    // 刷新 Receiver 实例
    pub fn refresh(&mut self) {
        let mut rng = rand::thread_rng(); // 创建随机数生成器
        self._okvsgen.clear(); // 清空 okvs 生成器
        self._pre_data.clear(); // 清空预处理数据
        for _ in 0..DIM {
            let mut pair = Vec::with_capacity(self.window as usize); // 为每一维度重新初始化预处理数据
            for _ in 0..self.window {
                let tem = Scalar::random(&mut rng); // 随机生成数据
                pair.push((tem, tem * self.sk)); // 存储数据和数据乘以私钥
            }
            self._pre_data.push(pair); // 更新预处理数据
            self._okvsgen.push(okvs::OkvsGen::new(self.window as u64)); // 更新 okvs 生成器
        }
    }

    /*
    为每个维度处理一组点集，并将结果编码为 okvs::Encoding 类型的向量
    */
    pub fn msg(&mut self, pt_set: &Vec<Point>) -> Vec<okvs::Encoding> {
        let mut result: Vec<okvs::Encoding> = Vec::with_capacity(DIM); // 初始化OKVS结果向量
        let mut list: Vec<(u64, (Scalar, Scalar))> = Vec::new(); // 初始化列表，存储键值对
                                                                 // 遍历每一个维度
        for i in 0..DIM {
            // 遍历每个接收者的点 pt
            // 将每个 Point 与 对应的预处理数目 对应
            for (pt, pre_window) in pt_set
                .iter()
                .zip(self._pre_data[i].windows(self.window).step_by(self.window))
            {
                let min: u64 = pt[i] - R as u64;
                for (j, pre_val) in pre_window.iter().enumerate() {
                    let key_ij = hash64(&(min + j as u64)); // 计算每个点的哈希值
                    list.push((key_ij, *pre_val)); // 将键值对添加到列表中
                }
            }
            result.push(self._okvsgen[i].encode(&list)); // 对每个维度的数据进行编码
            list.clear(); // 清空列表
        }
        self._okvsgen.clear(); // 清空 okvs 生成器
        self._pre_data.clear(); // 清空预处理数据
        return result; // 返回编码结果
    }

    // 后处理操作，检查消息是否有效
    #[inline]
    pub fn post_process(&mut self, msg_sender: &okvs::Encoding) -> u32 {
        for (u, v) in msg_sender.iter() {
            if (self.sk * u) == *v {
                return 1; // 如果匹配，返回 1
            }
        }
        return 0; // 否则返回 0
    }

    // 输出操作，返回计数
    pub fn output(&mut self, msg_sender: &okvs::Encoding, window: usize) -> u32 {
        let mut count = 0; // 初始化计数器
        for values in msg_sender.windows(window).step_by(window) {
            // 使用滑动窗口
            for (u, v) in values.iter() {
                if (self.sk * u) == *v {
                    count += 1; // 匹配时计数加 1
                    break;
                }
            }
        }
        return count; // 返回计数
    }
}

pub struct Sender {
    m: u64,                                                // 发送者的消息数量
    window: usize,                                         // 窗口大小
    pk: RistrettoPoint,                                    // 公钥
    _coins: Vec<(RistrettoPoint, RistrettoPoint, Scalar)>, // 存储coin的向量，包含 RistrettoPoint 和 Scalar
}

impl Sender {
    // 1 vs 1 test new
    pub fn new(pk_rec: RistrettoPoint) -> Self {
        let m: u64 = 1; // 计算消息数量
        let mut rng = rand::thread_rng(); // 创建随机数生成器
        let pk: RistrettoPoint = pk_rec; // 设置公钥
        let mut _coins: Vec<(RistrettoPoint, RistrettoPoint, Scalar)> =
            Vec::with_capacity(m as usize); // 初始化coin向量

        for _ in 0..m {
            // 生成coin
            let a = Scalar::random(&mut rng); // 随机生成一个 Scalar
            let b = Scalar::random(&mut rng); // 随机生成另一个 Scalar

            // _coins.0=a,_coins.1=a*pk,_coins.2=b
            _coins.push((&a * RISTRETTO_BASEPOINT_TABLE, a * pk_rec, b)); // 将coin添加到向量中
        }

        return Sender {
            // 返回 Sender 实例
            m,
            window: 1,
            pk,
            _coins,
        };
    }

    // 返回输出的大小
    pub fn get_output_size(&self) -> u64 {
        return self.m; // 返回消息数量
    }

    // 返回窗口大小
    pub fn get_windowsize(&self) -> usize {
        return self.window; // 返回窗口大小
    }

    // 刷新 Sender 实例
    pub fn refresh(&mut self) {
        let mut rng = rand::thread_rng(); // 创建随机数生成器
        self._coins.clear(); // 清空coin向量
        for _ in 0..self.m {
            // 重新生成coin
            let a = Scalar::random(&mut rng); // 随机生成一个 Scalar
            self._coins.push((
                &a * RISTRETTO_BASEPOINT_TABLE, // 计算coin
                a * self.pk,                    // 计算coin的公钥
                Scalar::random(&mut rng),       // 随机生成一个 Scalar
            ));
        }
    }

    // 发送单个消息
    #[inline]
    pub fn send_msg_single(
        &self,
        encodings: &Vec<okvs::Encoding>, // 编码向量
        pt: &Point,                      // 点
        index: usize,                    // 索引
    ) -> okvs::Encoding {
        let coin_window = &self._coins[index..index + self.window]; // 获取当前窗口的coin
        let mut uv: okvs::Encoding =
            vec![(RistrettoPoint::identity(), RistrettoPoint::identity()); 1]; // 初始化结果编码
        let mut tem: okvs::PointPair; // 临时变量

        // 遍历每个可能的块
        // 因为在recv计算用的是对应Point的block, sender则不知道，到底是哪个block
        for (i, coins) in coin_window.iter().enumerate() {
            for j in 0..DIM {
                let key_ij = hash64(&(pt[j] as u64)); // 计算每个点的哈希值
                tem = okvs::okvs_decode(&encodings[j], key_ij); // 解码
                uv[i].0 += tem.0; // 更新结果
                uv[i].1 += tem.1; // 更新结果
            }
            // 完成计算
            // uv[i].0= b*x+a
            // uv[i].1= b*(x*sk)+a*pk
            uv[i].0 = coins.2 * uv[i].0 + coins.0; // 最终计算
            uv[i].1 = coins.2 * uv[i].1 + coins.1; // 最终计算
        }
        return uv; // 返回编码结果
    }
}
