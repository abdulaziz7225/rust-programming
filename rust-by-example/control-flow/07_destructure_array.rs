fn main() {
    let array = [3, -2, 6, 10];
    match array {
        [0, second, third, _] => println!("array[0] = 0, array[1] = {second}, array[2] = {third}"),
        [1, _, third, _] => println!("array[0] = 1, array[1] was ignored, array[2] = {third}"),
        [-1, second, ..] => {
            println!("array[0] = -1, array[1] = {second}, array[2] was ignored or doesn't matter")
        }
        [3, second, tail @ ..] => {
            println!("array[0] = 3, array[1] = {second}, and the other elements were {tail:?}")
        }
        [first, middle @ .., last] => {
            println!("array[0] = {first}, middle = {middle:?}, array[2] = {last}")
        }
    }
}
