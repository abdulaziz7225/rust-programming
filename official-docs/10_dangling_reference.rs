fn main() {
    let reference_to_nothing = dangle();
    let ownership_is_moved_out = no_dangle();
}

fn dangle() -> &String {
    let s = String::from("Hello");
    &s
}

fn no_dangle() -> String {
    let s = String::from("Rust is the best");
    s
}
