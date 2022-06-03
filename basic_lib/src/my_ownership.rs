#[test]
fn ownership_test() {
    let s = String::from("Hello World");
    take_ownership(s);

    let x = 5;
    makes_copy(x);

    println!("x: {}", x);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}
