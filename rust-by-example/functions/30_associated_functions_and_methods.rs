#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}
impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x2 - x1) * (y2 - y1)).abs()
    }
    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p1.y += y;

        self.p2.x += x;
        self.p2.y += y;
    }
}

#[derive(Debug)]
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({first}, {second})");
    }
}

fn main() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    println!("Rectangle perimeter : {}", rectangle.perimeter());
    println!("Rectangle area : {}\n", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(5.0, 5.0),
    };
    // Error! 'rectangle' is immutable, but this method requires a mutable object
    rectangle.translate(2.0, 2.0);
    println!("Square location : {square:#?}");
    square.translate(1.0, 1.0);
    println!("Square location : {square:#?}\n");

    let pair = Pair(Box::new(1), Box::new(2));
    println!("Pair : {pair:#?}");
    pair.destroy();
    // Error! Previous 'destroy' call "consumed" 'pair'
    pair.destroy();
}
