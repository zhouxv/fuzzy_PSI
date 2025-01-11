#![allow(dead_code)]

extern crate f_psi;

use curve25519_dalek::scalar::Scalar;
use f_psi::okvs;
use fxhash::hash64;

use criterion::{criterion_group, criterion_main, Criterion};

// OKVS Bench!
pub fn bench1(b: &mut Criterion) {
    let n = 327680;
    let mut list: Vec<(u64, (Scalar, Scalar))> = Vec::new();
    println!("{} items, OkvsGen.Decode", n);
    for j in 0..n {
        list.push((hash64(&j), (Scalar::ONE, Scalar::ONE)));
    }

    b.bench_function("bench_okvs_decode", |b| {
        b.iter(|| {
            let mut okvs_instance = okvs::OkvsGen::new(n);
            let data = okvs_instance.encode(&list);
            let keys: Vec<u64> = (0..n).collect();
            okvs::okvs_decode_batch(&data, &keys);
        })
    });
}

// pub fn bench2(b: &mut Criterion) {
//     let n = 2621440;
//     let mut list: Vec<(u64, (Scalar, Scalar))> = Vec::new();
//     println!("{} items, OkvsGen.Decode", n);
//     for j in 0..n {
//         list.push((hash64(&j), (Scalar::ONE, Scalar::ONE)));
//     }

//     b.bench_function("bench_okvs_decode", |b| {
//         b.iter(|| {
//             let mut okvs_instance = okvs::OkvsGen::new(n);
//             let data = okvs_instance.encode(&list);
//             let keys: Vec<u64> = (0..n).collect();
//             okvs::okvs_decode_batch(&data, &keys);
//         })
//     });
// }

// pub fn bench3(b: &mut Criterion) {
//     let n = 83886080;
//     let mut list: Vec<(u64, (Scalar, Scalar))> = Vec::new();
//     println!("{} items, OkvsGen.Decode", n);
//     for j in 0..n {
//         list.push((hash64(&j), (Scalar::ONE, Scalar::ONE)));
//     }

//     b.bench_function("bench_okvs_decode", |b| {
//         b.iter(|| {
//             let mut okvs_instance = okvs::OkvsGen::new(n);
//             let data = okvs_instance.encode(&list);
//             let keys: Vec<u64> = (0..n).collect();
//             okvs::okvs_decode_batch(&data, &keys);
//         })
//     });
// }

// pub fn bench4(b: &mut Criterion) {
//     let n = 983040;
//     let mut list: Vec<(u64, (Scalar, Scalar))> = Vec::new();
//     println!("{} items, OkvsGen.Decode", n);
//     for j in 0..n {
//         list.push((hash64(&j), (Scalar::ONE, Scalar::ONE)));
//     }

//     b.bench_function("bench_okvs_decode", |b| {
//         b.iter(|| {
//             let mut okvs_instance = okvs::OkvsGen::new(n);
//             let data = okvs_instance.encode(&list);
//             let keys: Vec<u64> = (0..n).collect();
//             okvs::okvs_decode_batch(&data, &keys);
//         })
//     });
// }

// pub fn bench5(b: &mut Criterion) {
//     let n = 7864320;
//     let mut list: Vec<(u64, (Scalar, Scalar))> = Vec::new();
//     println!("{} items, OkvsGen.Decode", n);
//     for j in 0..n {
//         list.push((hash64(&j), (Scalar::ONE, Scalar::ONE)));
//     }

//     b.bench_function("bench_okvs_decode", |b| {
//         b.iter(|| {
//             let mut okvs_instance = okvs::OkvsGen::new(n);
//             let data = okvs_instance.encode(&list);
//             let keys: Vec<u64> = (0..n).collect();
//             okvs::okvs_decode_batch(&data, &keys);
//         })
//     });
// }

// pub fn bench6(b: &mut Criterion) {
//     let n = 251658240;
//     let mut list: Vec<(u64, (Scalar, Scalar))> = Vec::new();
//     println!("{} items, OkvsGen.Decode", n);
//     for j in 0..n {
//         list.push((hash64(&j), (Scalar::ONE, Scalar::ONE)));
//     }

//     b.bench_function("bench_okvs_decode", |b| {
//         b.iter(|| {
//             let mut okvs_instance = okvs::OkvsGen::new(n);
//             let data = okvs_instance.encode(&list);
//             let keys: Vec<u64> = (0..n).collect();
//             okvs::okvs_decode_batch(&data, &keys);
//         })
//     });
// }

// pub fn bench7(b: &mut Criterion) {
//     let n = 1310720;
//     let mut list: Vec<(u64, (Scalar, Scalar))> = Vec::new();
//     println!("{} items, OkvsGen.Decode", n);
//     for j in 0..n {
//         list.push((hash64(&j), (Scalar::ONE, Scalar::ONE)));
//     }

//     b.bench_function("bench_okvs_decode", |b| {
//         b.iter(|| {
//             let mut okvs_instance = okvs::OkvsGen::new(n);
//             let data = okvs_instance.encode(&list);
//             let keys: Vec<u64> = (0..n).collect();
//             okvs::okvs_decode_batch(&data, &keys);
//         })
//     });
// }

// pub fn bench8(b: &mut Criterion) {
//     let n = 10485760;
//     let mut list: Vec<(u64, (Scalar, Scalar))> = Vec::new();
//     println!("{} items, OkvsGen.Decode", n);
//     for j in 0..n {
//         list.push((hash64(&j), (Scalar::ONE, Scalar::ONE)));
//     }

//     b.bench_function("bench_okvs_decode", |b| {
//         b.iter(|| {
//             let mut okvs_instance = okvs::OkvsGen::new(n);
//             let data = okvs_instance.encode(&list);
//             let keys: Vec<u64> = (0..n).collect();
//             okvs::okvs_decode_batch(&data, &keys);
//         })
//     });
// }

// pub fn bench9(b: &mut Criterion) {
//     let n = 335544320;
//     let mut list: Vec<(u64, (Scalar, Scalar))> = Vec::new();
//     println!("{} items, OkvsGen.Decode", n);
//     for j in 0..n {
//         list.push((hash64(&j), (Scalar::ONE, Scalar::ONE)));
//     }

//     b.bench_function("bench_okvs_decode", |b| {
//         b.iter(|| {
//             let mut okvs_instance = okvs::OkvsGen::new(n);
//             let data = okvs_instance.encode(&list);
//             let keys: Vec<u64> = (0..n).collect();
//             okvs::okvs_decode_batch(&data, &keys);
//         })
//     });
// }

// pub fn bench10(b: &mut Criterion) {
//     let n = 3932160;
//     let mut list: Vec<(u64, (Scalar, Scalar))> = Vec::new();
//     println!("{} items, OkvsGen.Decode", n);
//     for j in 0..n {
//         list.push((hash64(&j), (Scalar::ONE, Scalar::ONE)));
//     }

//     b.bench_function("bench_okvs_decode", |b| {
//         b.iter(|| {
//             let mut okvs_instance = okvs::OkvsGen::new(n);
//             let data = okvs_instance.encode(&list);
//             let keys: Vec<u64> = (0..n).collect();
//             okvs::okvs_decode_batch(&data, &keys);
//         })
//     });
// }

// pub fn bench11(b: &mut Criterion) {
//     let n = 31457280;
//     let mut list: Vec<(u64, (Scalar, Scalar))> = Vec::new();
//     println!("{} items, OkvsGen.Decode", n);
//     for j in 0..n {
//         list.push((hash64(&j), (Scalar::ONE, Scalar::ONE)));
//     }

//     b.bench_function("bench_okvs_decode", |b| {
//         b.iter(|| {
//             let mut okvs_instance = okvs::OkvsGen::new(n);
//             let data = okvs_instance.encode(&list);
//             let keys: Vec<u64> = (0..n).collect();
//             okvs::okvs_decode_batch(&data, &keys);
//         })
//     });
// }

// pub fn bench12(b: &mut Criterion) {
//     let n = 1006632960;
//     let mut list: Vec<(u64, (Scalar, Scalar))> = Vec::new();
//     println!("{} items, OkvsGen.Decode", n);
//     for j in 0..n {
//         list.push((hash64(&j), (Scalar::ONE, Scalar::ONE)));
//     }

//     b.bench_function("bench_okvs_decode", |b| {
//         b.iter(|| {
//             let mut okvs_instance = okvs::OkvsGen::new(n);
//             let data = okvs_instance.encode(&list);
//             let keys: Vec<u64> = (0..n).collect();
//             okvs::okvs_decode_batch(&data, &keys);
//         })
//     });
// }

criterion_group!(
    name=benches;
    config = Criterion::default().significance_level(0.01).sample_size(10);
    targets=
    bench1,);
criterion_main!(benches);
