/*
Closures are inherently flexible and will do what functionality requires to make the closure work
without annotation. This allows capturing to flexibly adapt to the use case, sometimes moving
and sometimes borrowing. Closures can capture variables
- by reference: &T
- by mutable reference: &mut T
- by value: T
*/

fn main() {
    use std::mem;
    let color = String::from("green");

    // By reference (default behavior)
    let print = || println!("color : {color}");
    print(); // Call the closure using the borrowed value
    let _reborrow = &color; // 'color' can be borrowed immutable again
    print();
    let _color_moved = color; // 'color' was moved out
    print();

    // By mutable reference
    let mut count = 0;
    println!("count : {count}");
    let mut inc = || {
        count += 1;
        println!("count : {count}");
    };
    inc();
    let _reborrow = &count; // immutable borrow occurs here
    inc();
    let _count_reborrowed = &mut count;
    println!("count : {count}");

    // A non-copy (move) type
    let movable = Box::new(3);
    println!("movable : {movable:?}");
    let consume = || {
        println!("movable : {movable:?}");
        mem::drop(movable);
    };
    consume();
    // 'consume' consumes the variable, so this can only be called once
    println!("movable : {movable:?}");
    consume();

    // 'move' keyword before vertical pipes forces closure to take ownership of captured variables
    let list = vec![1, 2, 3];
    let contains = move |element| list.contains(element);
    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // 'list' variable moved due to use in closure
    println!("There are {} elements in the list", list.len());
}
