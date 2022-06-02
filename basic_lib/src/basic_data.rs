fn basic_fun() {
    const MAX_POINTS: u32 = 100000;

    let guess: u32 = "42".parse().expect("Not a number");

    println!("guess={}", guess);

    // 浮点数
    let x = 2.0;
    let y: f32 = 3.0;

    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    dbg!(
        "five_hundred={},six_point_four={},one={}",
        five_hundred,
        six_point_four,
        one
    );

    // 切片
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
}

#[test]
fn basic_fun_test(){
    basic_fun()
}