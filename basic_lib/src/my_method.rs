struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // 关联函数的使用方法跟构造器非常类似
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn premiter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // 该方法要求调用者是可变的，`&mut self` 是 `self: &mut Self` 的语法糖
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` 持有两个分配在堆上的整数
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroy Pair({},{})", first, second);
    }
}

#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

impl TrafficLightColor {
    fn color(&self) -> String {
        match *self {
            TrafficLightColor::Red => "red".to_string(),
            TrafficLightColor::Yellow => "yellow".to_string(),
            TrafficLightColor::Green => "green".to_string(),
        }
    }
}

#[test]
fn impl_test() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // 方法才是通过点操作符调用
    println!("Rectangle perimeter: {}", rectangle.premiter());
    println!("Rectangle area: {}", rectangle.area());

    // 关联函数的调用不是通过点操作符，而是使用 `::~
    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 2.0);
    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    let c = TrafficLightColor::Yellow;
    assert_eq!(c.color(), "yellow");
    println!("{:?}", c);
}
