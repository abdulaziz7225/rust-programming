fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {boxed_i32}");
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is {borrowed_i32}");
}

fn main() {
    // Create a boxed i32 in the heap, and a i32 on the stack
    // Remember: numbers can have arbitrary underscores added for readability
    // 5_i32 is the same as 5i32
    let boxed_i32 = Box::new(5______________i32);
    let stacked_i32 = 6__i32;
    
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;

        // Can't destroy `boxed_i32` while the inner value is borrowed later in scope.
        eat_box_i32(boxed_i32);

        borrow_i32(_ref_to_i32);
    }

    eat_box_i32(boxed_i32);
}
