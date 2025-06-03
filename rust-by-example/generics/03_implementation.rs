// struct S;
// struct GenericValue<T>(T);

// impl GenericValue<f32> {} // Specify "f32"
// impl GenericValue<S> {} // Specify "S" as defined above

// impl<T> GenericValue<T> {} // <T> must precede the type to remain generic

struct Value {
    val: f64,
}

struct GenericValue<T> {
    generic_val: T,
}

// impl of Value
impl Value {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// impl of GenericValue for a generic type "T"
impl<T> GenericValue<T> {
    fn value(&self) -> &T {
        &self.generic_val
    }
}

fn main() {
    let x = Value { val: 3.0 };
    let y = GenericValue { generic_val: 45i32};

    println!("{}, {}", x.value(), y.value());
}