use f_psi::okvs;
use f_psi::protocol;
use f_psi::psi;

use rand::Rng;
use std::time::Instant;

// 随机采集num个点
fn sample_test_data_points(num: usize) -> Vec<psi::Point> {
    let mut rng = rand::thread_rng();
    let mut points: Vec<psi::Point> = Vec::with_capacity(num);
    for _ in 0..num {
        let mut point: psi::Point = [0u64; psi::DIM];
        for i in 0..psi::DIM {
            point[i] = rng.gen_range(psi::SIDE_LEN..=(1 << 31));
        }
        points.push(point);
    }
    return points;
}

pub fn protocol() {
    let n = 2000;
    let m = 10000;
    println!("n: {}, m: {}, d:{}, delta:{}", n, m, psi::DIM, psi::R);
    let data_r = sample_test_data_points(n);
    let mut data_s = sample_test_data_points(m);
    data_s[9][0] = data_r[7][0] - psi::R / 2;
    data_s[9][1] = data_r[7][1] + psi::R / 2;
    data_s[11][0] = data_r[7][0] + psi::R;
    data_s[11][1] = data_r[7][1] - psi::R;
    let (rcr, sdr) = protocol::setup(n, m, false, 0);
    let now = Instant::now();

    // 测试时间
    protocol::run_standard(rcr, sdr, data_r, data_s);

    let elapsed = now.elapsed();
    println!("Elapsed Time for Protocol: {:.2?}", elapsed);
}

pub fn protocol_apart() {
    let n = 2000;
    let m = 1000;
    println!("n: {}, m: {}, d:{}, delta:{}", n, m, psi::DIM, psi::R);
    let data_r = sample_test_data_points(n);
    let mut data_s = sample_test_data_points(m);
    data_s[9][0] = data_r[7][0] - psi::R / 2;
    data_s[9][1] = data_r[7][1] + psi::R / 2;
    data_s[11][0] = data_r[7][0] + psi::R;
    data_s[11][1] = data_r[7][1] - psi::R;
    let (rcr, sdr) = protocol::setup(n, m, true, 0);

    let now = Instant::now();

    // 测试时间
    protocol::run_standard_apart(rcr, sdr, data_r, data_s);

    let elapsed = now.elapsed();
    println!("Elapsed Time for Protocol: {:.2?}", elapsed);
}

pub fn protocol_lp() {
    let n = 2000;
    let m = 1000;
    println!("n: {}, m: {}, d:{}, delta:{}", n, m, psi::DIM, psi::R);
    let data_r = sample_test_data_points(n);
    let mut data_s = sample_test_data_points(m);
    data_s[9][0] = data_r[7][0] - psi::R / 2;
    data_s[9][1] = data_r[7][1] + psi::R / 2;
    data_s[11][0] = data_r[7][0] + psi::R;
    data_s[11][1] = data_r[7][1] - psi::R;
    let (rcr, sdr) = protocol::setup(n, m, true, 2);

    let now = Instant::now();
    // 测试时间
    protocol::run_standard_lp(rcr, sdr, data_r, data_s, 2);
    let elapsed = now.elapsed();

    println!("Elapsed Time for Protocol: {:.2?}", elapsed);
}

// apart模式
pub fn psi_lp() {
    let n = 32; // 设置数据集R的大小
    let m = 1000000; // 设置数据集S的大小
    let data_r = sample_test_data_points(n); // 生成R的数据点
    let mut data_s = sample_test_data_points(m); // 生成S的数据点
    data_s[9][0] = data_r[7][0] - psi::R / 2; // 修改S的第10个点
    data_s[9][1] = data_r[7][1] + psi::R / 2; // 修改S的第10个点
    data_s[11][0] = data_r[7][0] + psi::R; // 修改S的第12个点
    data_s[11][1] = data_r[7][1] - psi::R; // 修改S的第12个点

    println!("n: {}, m: {}, d:{}, delta:{}", n, m, psi::DIM, psi::R); // 打印参数信息

    println!("开始创建角色实例");
    let create_now = Instant::now(); // 记录当前时间

    let mut rec_instance = psi::Receiver::new(n as u64, true); // 创建接收者实例
    let send_instance = psi::Sender::new(m as u64, rec_instance.publish_pk(), true, 0); // 创建发送者实例

    let create_elapsed = create_now.elapsed(); // 计算初始化时间
    println!(
        "{} items, Elapsed Time for Encoding (optimize=0): {:.2?}",
        rec_instance.get_output_size_per_dim() * psi::DIM as u64, // 打印编码的项目数量
        create_elapsed                                            // 打印编码耗时
    );

    println!("开始编码: rec_instance.msg_apart(&data_r)");
    let now = Instant::now(); // 记录当前时间

    let msg1 = rec_instance.msg_apart(&data_r); // 编码R的数据点

    let elapsed = now.elapsed(); // 计算编码时间

    println!("开始解码: rec_instance.msg_apart(&data_r)");
    println!(
        "{} items, Elapsed Time for Encoding (optimize=0): {:.2?}",
        rec_instance.get_output_size_per_dim() * psi::DIM as u64, // 打印编码的项目数量
        elapsed                                                   // 打印编码耗时
    );

    let sendnow = Instant::now(); // 记录发送开始时间
    let mut msgvec: Vec<okvs::PointPair> = Vec::with_capacity(m); // 创建消息向量
    for i in 0..m {
        msgvec.push(send_instance.send_msg_single_apart(&msg1, &data_s[i], i)); // 发送每个数据点
    }
    let sendelapsed = sendnow.elapsed(); // 计算发送时间
    println!(
        "{} items, Elapsed Time for Decoding (optimize=0): {:.2?}",
        send_instance.get_output_size(), // 打印解码的项目数量
        sendelapsed                      // 打印解码耗时
    );

    let mut c = 0u32; // 初始化计数器

    let recnow = Instant::now(); // 记录接收开始时间
                                 // let out = rec_instance.output(&msg2, send_instance.get_windowsize()); // 注释掉的输出行
    for i in 0..m {
        c += rec_instance.post_process_apart(&msgvec[i]); // 处理接收到的消息
    }
    let recoutputelapsed = recnow.elapsed(); // 计算接收处理时间
    println!(
        "{} items, Elapsed Time for Finishing (optimize=0): {:.2?}",
        send_instance.get_output_size(), // 打印完成的项目数量
        recoutputelapsed                 // 打印完成耗时
    );
    println!("Total : {:.2?}", now.elapsed()); // 打印总耗时
    println!("out: {}", c); // 打印最终输出
}

fn main() {
    println!("protocol()--------------------------------------------");
    protocol();

    // println!("protocol_apart()--------------------------------------");
    // protocol_apart();

    // println!("protocol_lp()-----------------------------------------");
    // protocol_lp();

    // println!("psi_lp()----------------------------------------------");
    // psi_lp();
}
