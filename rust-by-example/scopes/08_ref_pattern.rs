#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let c = 'Q';

    // A `ref` borrow on the left side of an assignment is equivalent to an `&` borrow on the right side.
    let ref ref_c1 = c;
    let ref_c2 = &c;
    println!("ref_c1 equals to ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 1, y: 2 };
    // `ref` is also valid when destructuring a struct.
    let _copy_of_x = {
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;
        *ref_to_x
    };

    let _copy_of_y = {
        let Point { x: _, y: ref_to_y } = &point;
        *ref_to_y
    };
    println!("_copy_of_x: {}, _copy_of_y: {}", _copy_of_x, _copy_of_y);

    // A mutable copy of `point`
    let mut mutable_point = point;
    {
        let Point {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point;
        *mut_ref_to_y += 1;
    }
    println!("point: ({}, {})", point.x, point.y);
    println!("mutable_point: ({}, {})", mutable_point.x, mutable_point.y);

    // A mutable tuple that includes a pointer
    let mut mutable_tuple = (Box::new(5u32), 3u32);
    println!("mutable tuple is {:?}", mutable_tuple);
    {
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    println!("mutable tuple is {:?}", mutable_tuple);
}
