fn main() {
    enum Color {
        Red,
        Blue,
        Green,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }
    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {r}, green: {g}, blue: {b}"),
        Color::HSV(h, s, v) => println!("Hue: {h}, saturation: {s}, value: {v}"),
        Color::HSL(h, s, l) => println!("Hue: {h}, saturation: {s}, lightness: {l}"),
        Color::CMY(c, m, y) => println!("Cyan: {c}, magenta: {m}, yellow: {y}"),
        Color::CMYK(c, m, y, k) => {
            println!("Cyan: {c}, magenta: {m}, yellow: {y}, key (black): {k}")
        }
    }
}
