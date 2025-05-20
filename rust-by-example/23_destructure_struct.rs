#[derive(Debug)]
struct Foo {
    x: (u32, u32),
    y: u32,
}

#[derive(Debug)]
struct Bar {
    foo: Foo,
}

fn main() {
    let foo = Foo { x: (7, 2), y: 2 };
    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i } => println!("y is 2, x = {i:?}"),
        Foo { y, .. } => println!("y = {y}, x doesn't matter"),
    }

    // You do not need a match block to destructure structs:
    let faa = Foo { x: (1, 2), y: 3 };
    let Foo { x: x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");

    let bar = Bar { foo: faa };
    let Bar {
        foo: Foo {
            x: nested_x,
            y: nested_y,
        },
    } = bar;
    println!("Bar: {bar:#?}");
    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
}
