// A module named 'my_mod'
mod my_mod {
    // Items in modules default to private visibility
    fn private_function() {
        println!("called 'my_mod::private_function()'");
    }

    pub fn public_function() {
        println!("called 'my_mod::public_function()'");
        nested::public_function_in_my_mod();
    }

    pub fn indirect_access() {
        println!("called 'my_mod::indirect_access()'");
        private_function();
    }

    // Modules can also be nested
    pub mod nested {
        pub fn public_function() {
            println!("called 'my_mod::nested::public_function()'");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called 'my_mod::nested::private_function()'");
        }

        // Functions declared using `pub(in path)` syntax are only visible
        // within the given path. `path` must be a parent or ancestor module
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called 'my_mod::nested::public_function_in_my_mod()', that\n> ");
            public_function_in_nested();
        }

        // Function declared using 'pub(self)' syntax are only visible within the current module,
        // which makes them as private functions with default visibility
        pub(self) fn public_function_in_nested() {
            println!("called 'my_mod::nested::public_function_in_nested()'");
        }

        // Functions declared using 'pub(super)' syntax are only visible within the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called 'my_mod::nested::public_function_in_super_mod()'");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called 'my_mod::call_public_function_in_my_mod()', that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called 'my_mod::public_function_in_crate()'");
    }

    mod private_nested_mod {
        #[allow(dead_code)]
        pub fn public_function() {
            println!("called 'my_mod::private_nested_mod::public_function()'");
        }

        // Private parent items will still restrict the visibility of a child item,
        // even if it is declared as visible within a bigger scope
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called 'my_mod::private_nested_mod::restricted_function()'");
        }
    }
}

fn public_function() {
    println!("called 'public_function()'");
}

fn main() {
    // Modules allow disambiguation between items that have the same name
    public_function();
    my_mod::public_function();
    println!();

    // Public items, including those inside nested modules, can be accessed from outside the parent module
    my_mod::indirect_access();
    println!();
    my_mod::nested::public_function();
    println!();
    my_mod::call_public_function_in_my_mod();
    println!();

    // pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();
    println!();

    // pub(in path) items can only be called from within the module specified
    my_mod::nested::public_function_in_my_mod();
    println!();

    // Private items of a module cannot be accessed directly, even if nested in a public module
    my_mod::private_function(); // Error! 'private_function' is private
    my_mod::nested::private_function(); // Error! 'private_function' is private
    my_mod::private_nested_mod::public_function(); // Error! 'private_nested_mod' is a private module
    my_mod::private_nested_mod::restricted_function(); // Error! 'private_nested_mod' is a private module
}