struct A;
struct Single(A);
struct SingleGeneric<T>(T);

fn main() {
    let _s = Single(A);
    let _char: SingleGeneric<char> = SingleGeneric('a');

    let _t = SingleGeneric(A);
    let _i32 = SingleGeneric(6);
    let _char = SingleGeneric('a');
}
