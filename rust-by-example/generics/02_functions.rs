struct A;
struct Struct(A);
struct StructGeneric<T>(T);

fn regular_function(_s: Struct) {}

fn generic_specified_type(_s: StructGeneric<A>) {}

fn generic_specified_i32(_s: StructGeneric<i32>) {}

fn generic<T>(_s: StructGeneric<T>) {}

fn main() {
    regular_function(Struct(A)); // Concrete function
    generic_specified_type(StructGeneric(A)); // Implicitly specified type parameter "A"
    generic_specified_i32(StructGeneric(435)); // Implicitly specified type parameter "i32"
    generic::<char>(StructGeneric('a')); // Explicitly specified type parameter "char"
    generic(StructGeneric('c')); // Implicitly specified type parameter "char"
}
