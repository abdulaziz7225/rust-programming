// Unit struct
struct Unit;

// Tuple struct
struct Pair(i32, f32);

// Struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("Peter : {peter:#?}");

    // Instantiate Point
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point = Point { x: 10.3, y: 0.2 };
    let third_point: Point = Point {
        x: 20.5,
        ..another_point
    };
    println!("point coordinates: ({}, {})", point.x, point.y);
    println!(
        "another_point coordinates: ({}, {})",
        another_point.x, another_point.y
    );
    println!(
        "third_point coordinates: ({}, {})",
        third_point.x, third_point.y
    );

    // Instantiate Rectangle
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;
    let rectangle: Rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: another_point,
    };
    println!("rectangle coordinates : {:#?}", rectangle);

    // Instantiate tuple struct
    let pair = Pair(12, 56.971);
    println!("Pair contains {:?} and {:?}", pair.0, pair.1);
    let Pair(integer, decimal) = pair;
    println!("Pair contains : {integer} and {decimal}");
}
