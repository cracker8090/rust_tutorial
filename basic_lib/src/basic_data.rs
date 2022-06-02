fn basic_fun() {
    const MAX_POINTS: u32 = 100000;

    let guess: u32 = "42".parse().expect("Not a number");

    println!("guess={}", guess);

    // 浮点数
    let x = 2.0;
    let y: f32 = 3.0;

    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup:{:?}", tup);
    let (x, y, z) = tup;
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!(
        "five_hundred={},six_point_four={},one={}",
        five_hundred, six_point_four, one
    );

    // 切片
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
}

#[test]
fn basic_fun_test() {
    use std::mem::size_of_val;
    basic_fun();
    let x = 'c';
    println!("size_of_val start,{}", size_of_val(&x));
    assert_eq!(size_of_val(&x), 4);
    println!("size_of_val Success!");
}

#[test]
fn float_test() {
    // let x: f64 = 0.3; 浮点数比较
    assert_ne!(0.1 + 0.2, 0.3);
    assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.00001);
}

#[test]
fn NaN_test() {
    // is_nan判断
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("undefined math");
    }
}

#[test]
fn Complex_test() {
    // 复数
    // num crate: https://crates.io/crates/num
    use num::complex::Complex;
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
}

#[test]
fn unit_test() {
    // () 单元类型
    use std::mem::size_of_val;
    let unit: () = ();
    println!("size_of_val(&unit)={}", size_of_val(&unit));
    assert!(size_of_val(&unit) == 0);

    println!("Success!")
}

// fn main() {
//     println!("Hello, world!");
//     let a = 10;
//     let b: i32 = 20;

//     let mut c = 30i32;
//     let d = 30_i32;
//     let e = add(add(a, b), add(c, d));

//     println!("(a+b) + (c+d) = {}", e);
// }

// fn add(i: i32, j: i32) -> i32 {
//     i + j
// }

// fn main() {
//     let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
//     let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

//     println!("abc (f32)");
//     println!("     0.1 + 0.2: {:?}", (abc.0 + abc.1).to_bits());
//     println!("           0.3: {:?}", (abc.2).to_bits());
//     println!();

//     println!("xyz (f64)");
//     println!("     0.1 + 0.2: {:?}", (xyz.0 + xyz.1).to_bits());
//     println!("           0.3: {:?}", (xyz.2).to_bits());
//     println!();

//     assert!(abc.0 + abc.1 == abc.2);
//     assert!(xyz.0 + xyz.1 == xyz.2);
// }
