struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str {
    "red"
}
fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));

    // red() function does not work with 'blue_jay' variable and blue() function with 'cardinal'
    println!("A blue jay is {}", red(&blue_jay));
    println!("A cardinal is {}", blue(&cardinal));

    // 'Red' trait is not implemented for 'Turkey' struct
    println!("A turkey is {}", red(&_turkey));
}
