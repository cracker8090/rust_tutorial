use std::fs::File;
use std::io::ErrorKind;
#[test]
fn panic_test() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];

    // v[99];

    // let f = File::open("hello1.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error);
    //     }
    // };

    let f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}
// 测试模块
#[cfg(test)]
mod test {
    use super::add_two;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}
// 测试属性
#[test]
fn it_work() {
    assert!(true);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn it_works() {
    assert_eq!("Hello", "world");
}

// 测试目录 新建一个tests目录
