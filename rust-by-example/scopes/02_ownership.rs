fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {c}");
}

fn main() {
    // _Stack_ allocated integer
    let x = 5u32;
    // *Copy* 'x' into 'y' - no resources are moved
    let y = x;
    println!("x is {x}, and y is {y}");

    // 'a' is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);
    println!("a contains: {a}");
    let b = a;
    
    println!("a contains: {a}");
    destroy_box(b);
    println!("b contains: {b}");
}
