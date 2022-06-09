use std::collections::HashMap;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}

#[test]
fn hashmap_test() {
    let rect = Rectangle {
        width: 30,
        length: 50,
    };

    // 打印
    println!("{}", area(&rect));
    println!("{:#?}", rect);

    let mut my_gems = HashMap::new();
    my_gems.insert("红宝石", 1);
    my_gems.insert("蓝宝石", 2);
    my_gems.insert("绿宝石", 18);
    println!("{:#?}", my_gems);
}

#[test]
fn vec_test() {
    let v: Vec<i32> = Vec::new();
    let mut p = Vec::new();
    p.push(1);

    let ve = vec![1, 2, 3, 4, 5];
    let third = ve[2];

    // &v[2] 表示借用 v 中的第三个元素，最终会获得该元素的引用。
    // 而 v.get(2) 也是访问第三个元素，但是有所不同的是，它返回了 Option<&T>，
    // 因此还需要额外的 match 来匹配解构出具体的值。
    match ve.get(2) {
        Some(third) => println!("third is {}", third),
        None => println!("nothing!"),
    }
}

trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0)
    }
}
struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0)
    }
}

#[test]
fn vec_diff_test() {
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}

#[test]
fn vec_to_hashmap_test() {
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let mut teams_map = HashMap::new();
    for team in &teams_list {
        teams_map.insert(&team.0, &team.1);
    }
    println!("{:?}", teams_map);

    // 先将 Vec 转为迭代器，接着通过 collect 方法，将迭代器中的元素收集后，转成 HashMap
    let teams_map2: HashMap<_, _> = teams_list.into_iter().collect();

    println!("{:?}", teams_map2);
}

#[test]
fn hashmap_curd_test() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);
    // println!("score={}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow".to_string()).or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow".to_string()).or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入
}

fn test_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("yellow"), 10);
    scores.insert(String::from("yellow"), 20);
    scores.entry(String::from("blue")).or_insert(50);
    scores.entry(String::from("yellow")).or_insert(50);
    println!("{:?}", scores);
}
