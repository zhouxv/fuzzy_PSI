use f_psi::psi;
use f_psi::psi_test;
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

pub fn fuzzy_macthing_infinity(setting: bool) {
    let data_r = sample_test_data_points(1);
    let mut data_s = sample_test_data_points(1);
    if setting {
        data_s[0][0] = data_r[0][0] - psi::R / 2;
        data_s[0][1] = data_r[0][1] + psi::R / 2;
    }

    println!("r: {:?}, s: {:?}", data_r, data_s);

    let mut psi_rec = psi_test::Receiver::new(); // 创建Recv
    let psi_sed = psi_test::Sender::new(psi_rec.publish_pk());

    let msg1 = psi_rec.msg(&data_r);

    let msg: Vec<(
        curve25519_dalek::RistrettoPoint,
        curve25519_dalek::RistrettoPoint,
    )> = psi_sed.send_msg_single(&msg1, &data_s[0], 0); // 发送单个消息

    let res = psi_rec.post_process(&msg);
    println!("res: {:?}", res);
}

fn main() {
    println!("1vs1 fuzzymacth l∞()--------------------------------------------");
    fuzzy_macthing_infinity(true);

    fuzzy_macthing_infinity(false);

    // println!("protocol_apart()--------------------------------------");
    // protocol_apart();

    // println!("protocol_lp()-----------------------------------------");
    // protocol_lp();

    // println!("psi_lp()----------------------------------------------");
    // psi_lp();
}
