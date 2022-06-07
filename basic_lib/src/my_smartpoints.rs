use std::{cell::Cell, cell::RefCell, ops::Deref, rc::Rc, sync::Arc, thread};

// enum List {
//     Cons(i32, List),
//     Nil,
// }
enum List {
    Cons(i32, Box<List>),
    Nil,
}
// use crate::List::{Cons, Nil};

struct User {
    id: u32,
    age: Cell<u32>,
}

struct Hello {
    v: Box<i32>,
}

struct World {
    v: Rc<i32>,
}

fn add(i: &i32) {}

fn print_me(s: &String) {
    println!("{}", *s);
}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(v: T) -> MyBox<T> {
        MyBox(v)
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Drop MyBox!");
    }
}

#[test]
fn box_test() {
    // std::mem::drop 提早丢弃值
    // 每次调用
    // Rc::clone ， Rc 中数据的引用计数都会增加， 直到有零个引用之前其数据都不会被清理
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    {
        let mb = MyBox::new(20);
        let mc = *mb;
    }
}

#[test]
fn rc_test() {
    // Rc 多个拥有者,一个计数一个地址，多个只读所有者，非线程安全
    // 在rust中不应该说深拷贝和浅拷贝，就是Copy trait(通过赋值或参数传递来隐式复制)和
    // Clone trait(通过显式调用clone方法来复制)
    let r1 = Rc::new(20);
    add(&r1);

    let b1 = Box::new(10);
    add(&b1);

    let h1 = Hello { v: b1 };
    // let h2 = Hello{
    //     v:b1,
    // };
    let w2 = World { v: r1.clone() };
    let w1 = World { v: r1 };

    // Arc 跨线程安全
    // let ar1 = Rc::new(10);
    let ar1 = Arc::new(10);
    for _ in 0..10 {
        let r = ar1.clone();
        thread::spawn(move || {
            println!("{}", r);
        });
    }
}

#[test]
fn cell_test() {
    let foo = User {
        id: 1,
        age: Cell::new(3),
    };
    assert_eq!(1, foo.id);
    assert_eq!(3, foo.age.get());
    foo.age.set(5);
    assert_eq!(5, foo.age.get());

    let x = RefCell::new(vec![1, 2, 3]);
    println!("{:?}", x.borrow());
    x.borrow_mut().push(5);
    println!("{:?}", x.borrow());
}
