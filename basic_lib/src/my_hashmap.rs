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
