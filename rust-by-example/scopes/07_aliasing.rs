struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut point = Point { x: 1, y: 2, z: 3 };

    let borrowed_point = &point;
    let another_borrowed_point = &point;

    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrowed_point.y, point.z
    );

    // Error! Can't borrow `point` as mutable because it's currently borrowed as immutable.
    let mutable_borrow = &mut point;

    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrowed_point.y, point.z
    );

    let mutable_borrow = &mut point;
    mutable_borrow.x += 4;
    mutable_borrow.y += 3;
    mutable_borrow.z += 2;

    // Error! Can't borrow `point` as immutable because it's currently borrowed as mutable.
    let y = &point.y;
    
    // Error! Can't print because `println!` takes an immutable reference.
    println!("Point Z coordinate is: {}", point.z);

    println!(
        "Mutable borrow has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    // The mutable reference is no longer used for the rest of the code so it is possible to re-borrow
    let new_borrowed_point = &point;
    println!(
        "New borrowed point has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );
}
