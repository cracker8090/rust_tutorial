pub fn play(name: String) {
    println!("Playing basic test {} :basic-lib", name);
}

#[test]
fn play_test() {
    play("Play testing".to_string());
}

#[test]
fn play_result_test() -> Result<(), String> {
    if 1 + 2 == 3 {
        Ok(())
    } else {
        Err(String::from("play_result_test error"))
    }
}

fn move_ownership(s: &String) {
    println!("ownership of \"{}\" is moved here!", s)
}

fn greetings(s: &str) {
    println!("{}", s)
}

#[test]
fn string1_test() {
    // 可以使用 String::from 或 to_string 将 &str 转换成 String 类型
    let _s0 = "hello, world".to_string();

    let mut s: String = String::from("hello,");
    s.push_str("world");
    s.push('!');
    move_ownership(&s);

    assert_eq!(s, "hello,world!");

    let mut s1 = String::from("hello,world");
    let slice1: &str = &s1;
    assert_eq!(slice1, "hello,world");

    let slice2 = &s[0..5];
    assert_eq!(slice2, "hello");

    let slice3: &mut String = &mut s1;
    slice3.push('!');
    assert_eq!(slice3, "hello,world!");

    let mut s = String::new();
    println!("capacity:{}", s.capacity());
    // for _ in 0..2 {
    s.push_str("hello");
    println!("capacity:{}", s.capacity());
    // }

    // 要使用 str 类型，只能配合 Box。 & 可以用来将 Box<str> 转换为 &str 类型
    let s: Box<str> = "hello, world".into();
    greetings(&s);
}

#[test]
fn replace_test() {
    let s = String::from("I like dogs");
    // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
    // https://doc.rust-lang.org/std/string/struct.String.html
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");
}

#[test]
fn vec_test() {
    let sparkle_heart = vec![240, 159, 146, 150];

    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
    assert_eq!("💖", sparkle_heart);

    let bytes = sparkle_heart.into_bytes();
    assert_eq!(bytes, [240, 159, 146, 150]);
}

#[test]
fn string2_test() {
    // 只能将 String 跟 &str 类型进行拼接，并且 String 的所有权在此过程中会被 move
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);

    let h = &s1[0..1];
    assert_eq!(h, "h");
}

#[test]
fn vec2_test() {
    let v: Vec<i32> = Vec::new();
    let mut v1 = Vec::new();
    let v2 = vec![1, 2, 3];
    v1.push(1);
    println!("{} ----{:?}-----", v.len(), v1);
    println!("v2={}", v2[1]);
}

#[test]
fn array_test() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(arr.len(), 5);

    let arr1: [_; 3] = ['a', 'b', 'c'];
    assert!(std::mem::size_of_val(&arr1) == 12);

    let i: [i32; 100] = [1; 100];
    assert!(i[0] == 1);
    assert!(i.len() == 100);
}
