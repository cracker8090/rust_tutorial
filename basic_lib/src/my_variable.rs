use std::mem::size_of_val;

#[test]
fn variable_test() {
    // let x:i32 = 10;
    // {
    //     let y:i32 = 5;
    //     println!("x={},y={}",x,y);
    // }
    // println!("x={},y=", x);

    // let (mut x, y) = (1, 2);
    // x += 2;

    // assert_eq!(x, 3);
    // assert_eq!(y, 2);

    let x: i32 = 5;
    let mut y: u32 = 5;
    let z = 10;
    println!("x={},y={},z={}", x, y, z);
    println!("size={}", size_of_val(&x));

    let unit: () = ();
    assert!(size_of_val(&unit) == 0);
    // println!("sizeof(unit)={}", size_of_val(&unit));

    let v: u16 = 38_u8 as u16;
    println!("v={}", v);

    let s = sum(1, 2);
    assert_eq!(s, 3);

    let s1 = String::from("hello,world");
    let s2 = take_ownership(s1);
    println!("{}", s2);

    let t = (String::from("hello"), String::from("world"));
    // let _s = t.0;
    let (s1, s2) = &t;
    let p = &t;
    println!("{:?}, {:?}, {:?}", s1, s2, t);
    println!("x 的内存地址是 {:p}", p);

    // println!("{:?}", t.1);

    let x = 5;
    let y = &x;

    assert_eq!(5, *y);

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    let s: &str = "hello, world";

    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s2);

    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");

    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:#?}", too_long_tuple);

    let (x, y, z);
    (y, z, x) = (1, 2, 3);
}

fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}

fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}
