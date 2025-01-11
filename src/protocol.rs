use crate::psi; // 引入 psi 模块

use std::sync::mpsc as std_mpsc; // 引入标准多生产者-单消费者通道
use std::thread; // 引入线程模块

// 设置函数，返回 psi::Receiver 和 psi::Sender
pub fn setup(num_n: usize, num_m: usize, apart: bool, metric: u32) -> (psi::Receiver, psi::Sender) {
    let psi_rec = psi::Receiver::new(num_n as u64, apart); // 创建Recv
    let psi_sed = psi::Sender::new(num_m as u64, psi_rec.publish_pk(), apart, metric); // 创建Sender
    return (psi_rec, psi_sed); // 返回Recv和Sender
}

// 标准运行函数
pub fn run_standard(
    mut psi_rec: psi::Receiver, // Recv
    psi_sed: psi::Sender,       // Sender
    data_r: Vec<psi::Point>,    // 数据集 R
    data_s: Vec<psi::Point>,    // 数据集 S
) {
    let (done_tx, done_rx) = std_mpsc::channel::<()>(); // 创建完成信号通道
    let (sender, receiver) = std_mpsc::channel(); // 创建发送和接收通道

    let msg1 = psi_rec.msg(&data_r); // 获取消息

    // 发送线程
    thread::spawn(move || {
        for i in 0..data_s.len() {
            // 遍历数据集 S
            let msg: Vec<(
                curve25519_dalek::RistrettoPoint,
                curve25519_dalek::RistrettoPoint,
            )> = psi_sed.send_msg_single(&msg1, &data_s[i], i * psi::BLK_CELLS); // 发送单个消息
            if sender.send(msg).is_err() {
                // 如果发送失败
                println!("Receiver has been dropped!"); // 打印错误信息
                break; // 退出循环
            }
        }
    });

    // 接收线程
    thread::spawn(move || {
        let mut count = 0u32; // 初始化计数器
        for msg2 in receiver.iter() {
            // 遍历接收到的消息
            count += psi_rec.post_process(&msg2); // 处理消息并更新计数
        }
        println!("count: {}", count); // 打印计数
        done_tx.send(()).expect("Failed to send done signal"); // 发送完成信号
    });

    // 等待任务完成
    done_rx.recv().expect("Failed to receive done signal"); // 接收完成信号
}

// 标准apart运行函数
pub fn run_standard_apart(
    mut psi_rec: psi::Receiver, // Recv
    psi_sed: psi::Sender,       // Sender
    data_r: Vec<psi::Point>,    // 数据集 R
    data_s: Vec<psi::Point>,    // 数据集 S
) {
    let (done_tx, done_rx) = std_mpsc::channel::<()>(); // 创建完成信号通道
    let (sender, receiver) = std_mpsc::channel(); // 创建发送和接收通道

    let msg1 = psi_rec.msg_apart(&data_r); // 获取apart的消息

    // 发送线程
    thread::spawn(move || {
        for i in 0..data_s.len() {
            // 遍历数据集 S
            let msg = psi_sed.send_msg_single_apart(&msg1, &data_s[i], i); // 发送单个apart消息
            if sender.send(msg).is_err() {
                // 如果发送失败
                println!("Receiver has been dropped!"); // 打印错误信息
                break; // 退出循环
            }
        }
    });

    // 接收线程
    thread::spawn(move || {
        let mut count = 0u32; // 初始化计数器
        for msg2 in receiver.iter() {
            // 遍历接收到的消息
            count += psi_rec.post_process_apart(&msg2); // 处理消息并更新计数
        }
        println!("count: {}", count); // 打印计数
        done_tx.send(()).expect("Failed to send done signal"); // 发送完成信号
    });

    // 等待任务完成
    done_rx.recv().expect("Failed to receive done signal"); // 接收完成信号
}

// 标准 LP 运行函数
pub fn run_standard_lp(
    mut psi_rec: psi::Receiver, // 可变Recv
    psi_sed: psi::Sender,       // Sender
    data_r: Vec<psi::Point>,    // 数据集 R
    data_s: Vec<psi::Point>,    // 数据集 S
    metric: u32,                // 度量
) {
    let (done_tx, done_rx) = std_mpsc::channel::<()>(); // 创建完成信号通道
    let (sender, receiver) = std_mpsc::channel(); // 创建发送和接收通道

    let msg1 = psi_rec.lp_msg_apart(&data_r, metric); // 获取 LP 消息

    // 发送线程
    thread::spawn(move || {
        for i in 0..data_s.len() {
            // 遍历数据集 S
            let msg = psi_sed.lp_send_msg_single_apart(&msg1, &data_s[i], i); // 发送单个 LP 消息
            if sender.send(msg).is_err() {
                // 如果发送失败
                println!("Receiver has been dropped!"); // 打印错误信息
                break; // 退出循环
            }
        }
    });

    // 接收线程
    thread::spawn(move || {
        let mut count = 0u32; // 初始化计数器
        for msg2 in receiver.iter() {
            // 遍历接收到的消息
            count += psi_rec.lp_post_process_apart(&msg2); // 处理消息并更新计数
        }
        println!("Lp metric {}, count: {}", metric, count); // 打印度量和计数
        done_tx.send(()).expect("Failed to send done signal"); // 发送完成信号
    });

    // 等待任务完成
    done_rx.recv().expect("Failed to receive done signal"); // 接收完成信号
}
