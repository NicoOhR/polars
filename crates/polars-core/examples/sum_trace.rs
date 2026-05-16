use std::time::Instant;

use polars_core::prelude::*;

const N: usize = 10_000_000;
const ITERS: u32 = 10;

fn bench(label: &str, s: &Series) {
    let _ = s.sum::<i64>();

    let start = Instant::now();
    for _ in 0..ITERS {
        let _ = s.sum::<i64>();
    }
    let elapsed = start.elapsed();
    println!("{label}: avg {:.3}ms", elapsed.as_secs_f64() * 1000.0 / ITERS as f64);
}

fn main() {
    let i8_data: Vec<i8> = (0..N).map(|i| (i % 100) as i8).collect();
    let i8_data_nulls: Vec<Option<i8>> = (0..N)
        .map(|i| if i % 10 == 0 { None } else { Some((i % 100) as i8) })
        .collect();
    let u8_data: Vec<u8> = (0..N).map(|i| (i % 200) as u8).collect();
    let i16_data: Vec<i16> = (0..N).map(|i| (i % 1000) as i16).collect();
    let u16_data: Vec<u16> = (0..N).map(|i| (i % 1000) as u16).collect();
    let u64_data: Vec<u64> = (0..N).map(|i| i as u64).collect();

    let s_i8 = Series::new("i8".into(), i8_data.as_slice());
    let s_i8_nulls = Series::new("i8_nulls".into(), i8_data_nulls.as_slice());
    let s_u8 = Series::new("u8".into(), u8_data.as_slice());
    let s_i16 = Series::new("i16".into(), i16_data.as_slice());
    let s_u16 = Series::new("u16".into(), u16_data.as_slice());
    let s_u64 = Series::new("u64".into(), u64_data.as_slice());

    println!("N = {N}, averaged over {ITERS} iterations\n");

    bench("Int8  (no nulls)", &s_i8);
    bench("Int8  (nulls)   ", &s_i8_nulls);
    bench("UInt8 (no nulls)", &s_u8);
    bench("Int16 (no nulls)", &s_i16);
    bench("UInt16(no nulls)", &s_u16);
    bench("UInt64(no nulls)", &s_u64);
}
