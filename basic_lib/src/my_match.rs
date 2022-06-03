#[test]
fn match_test() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id @ 3..=7 } => println!("id 值的范围在 [3, 7] 之间: {}", id),
        Message::Hello {
            id: newid @ (10 | 11 | 12),
        } => {
            println!("id 值的范围在 [10, 12] 之间: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }

    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);
    match numbers {
        (first, .., last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }

    let mut v = String::from("hello,");
    let r = &mut v;
    match r {
        value => value.push_str(" world!"),
    }
}

enum Message {
    Hello { id: i32 },
}

#[test]
fn match2_test() {
    let n = 10;
    match n {
        // 匹配一个单独的值
        1 => println!("One!"),
        // 使用 `|` 填空，不要使用 `..` 或 `..=`
        __ => println!("match 2 -> 5"),
        // 匹配一个闭区间的数值序列
        6..=10 => {
            println!("match 6 -> 10")
        }
        _ => {
            println!("match 11 -> +infinite")
        }
    }
}
