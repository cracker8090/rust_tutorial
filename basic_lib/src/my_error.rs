use std::error::Error;
use std::fs::File;
// use serde::{Deserialize,Serialize};
// use serde_json::Result;

#[test]
fn basic_error_test() {
    println!(
        "{}",
        match find_store("IOS") {
            Some(s) => s,
            None => "Not a valid mobile OS",
        }
    );

    let json_string = r#"
    {
        "name":"John Doe,
        "age":43,
        "phones":[
            "+44 1234567",
            "+44 2345678",
        ]
    }"#;
    // let f = File::open("hello.txt")?;
    // 示例、代码原型和测试都非常适合 panic, unwrap 或 expect
    // ? 运算符作用于 File::open 返回的 Result 值，不过 main 函数的返回类型是 () 而不是 Result
    // unwrap隐含了panic!
    // let p:Person = match serde_json::from_str(json_string) {
    //     Ok(p) => p,
    //     Err(e) => panic!("cannot parse JSON {:?}",e),
    // }
    // let f = File::open("hello.txt").unwrap();
    let _f = File::open("hello.txt").expect("Failed to open hello.txt");
    // let _f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };
}

fn find_store(mobile_os: &str) -> Option<&str> {
    match mobile_os {
        "IOS" => Some("App Store"),
        "android" => Some("Play Store"),
        _ => None,
    }
}

// #[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}
