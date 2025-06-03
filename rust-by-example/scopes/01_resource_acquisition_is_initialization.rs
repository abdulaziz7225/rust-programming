fn create_box() {
    let _box1 = Box::new(3i32);
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped!");
    }
}

fn main() {
    let _box2 = Box::new(5i32);

    // A nested scope
    {
        let _box3 = Box::new(34i32);
    }

    for _ in 0u32..1_000 {
        create_box();
    }

    let x = ToDrop;
    println!("Made a ToDrop!");
}
