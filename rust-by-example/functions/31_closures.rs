fn main() {
    let outer_var = 42;

    // A regular function can't refer to variables in the enclosing environment
    // fn function(i: i32) -> i32 {
    //     i + outer_var
    // }

    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i| i + outer_var;

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(2));

    let one = || 1;
    println!("closure returning one: {}", one());
}
