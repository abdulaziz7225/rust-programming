pub fn public_function() {
    println!("called library's 'public_function()'");
}

fn private_function() {
    println!("called library's 'private_function()'");
}

pub fn indirect_access() {
    print!("called library's 'indirect_access()', that\n> ");
    
    private_function();
}