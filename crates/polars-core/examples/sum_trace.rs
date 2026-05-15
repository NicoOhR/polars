use polars_core::prelude::*;

fn main() {
    // small dtype with nulls — exercises the cast-to-i64 + validity mask path
    let s = Series::new(
        "x".into(),
        &[Some(1i8), None, Some(3), Some(4), None, Some(6)],
    );
    println!("--- sum of Int8 with nulls ---");
    let result: i64 = s.sum().unwrap();
    println!("result: {result}");

    // small dtype without nulls — exercises the no-validity path
    let s2 = Series::new("y".into(), &[1i8, 2, 3, 4, 5, 6]);
    println!("\n--- sum of Int8 without nulls ---");
    let result2: i64 = s2.sum().unwrap();
    println!("result: {result2}");
}
