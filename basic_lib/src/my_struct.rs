#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    // hobby: String,
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[test]
fn struct_test() {
    let age = 30;
    let mut p = Person {
        name: String::from("sunface"),
        age: age,
        // hobby: String::from("basketball"),
    };
    p.age = 18;
    p.name = String::from("sunfei");
    println!("{:#?}", p);

    let msg1 = Message::Move { x: 1, y: 2 };
    let msg2 = Message::Write("hello,world".to_string());
    dbg!(&msg1, &msg2);

    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg);
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(n) = six {
        println!("{}", n);
        // return;
    }
    // panic!("Do not run this code");

    let n = 25;
    let big_n = if n < 10 && n > -10 {
        println!(" 数字太小，先增加 10 倍再说");

        10 * n
    } else {
        println!("数字太大，我们得让它减半");

        n / 2
    };

    println!("{} -> {}", n, big_n);

    for n in 1..100 {
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
        // println!("{}", n);
    }

    let a = [4, 3, 2, 1];
    for (i, v) in a.iter().enumerate() {
        println!("The {}th element is {}", i + 1, v);
    }
}

fn build_person(name: String, age: u8) -> Person {
    Person { age, name }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}

fn show_message(msg: Message) {
    println!("{:?}", msg);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
