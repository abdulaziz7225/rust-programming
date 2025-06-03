#[allow(dead_code)]
enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    // // Since `match` keyword is exhaustive. we should cover all cases or possibilities
    // let optional = Some(7);
    // match optional {
    //     Some(i) => println!("This is a really long string and `{i}`"),
    //     _ => (),
    // }
    //
    // // if let
    // let number = Some(7);
    // let letter: Option<i32> = None;
    // let emoticon: Option<i32> = None;
    //
    // if let Some(i) = number {
    //     println!("Matched : {i}");
    // }
    //
    // if let Some(i) = letter {
    //     println!("Matched : {i}!");
    // } else {
    //     println!("Didn't match a number. Let's go with a letter");
    // }
    //
    // let i_like_letters = false;
    // if let Some(i) = emoticon {
    //     println!("Matched : {i}!!");
    // } else if i_like_letters {
    //     println!("Didn't match a number. Let's go with a letter!!");
    // } else {
    //     println!("I don't like letters. Let's go with an emoticon");
    // }
    //
    // // In the same way, if let can be used to match any enum value
    // let a = Foo::Bar;
    // let b = Foo::Baz;
    // let c = Foo::Qux(100);
    //
    // if let Foo::Bar = a {
    //     println!("a is foobar");
    // }
    // if let Foo::Bar = b {
    //     println!("b is foobar");
    // }
    // if let Foo::Qux(value) = c {
    //     println!("c is {value}");
    // }
    // if let Foo::Qux(value @ 100) = c {
    //     println!("c is one hundred");
    // }

    // enum doesn't implement or derive PartialEq

    // #[derive(PartialEq)]
    enum Fizz {Buzz}
    let x = Fizz::Buzz;
    if Fizz::Buzz == x {
        println!("x is foobar");
    }

    if let Fizz::Buzz = x {
        println!("x is foobar !!!");
    }
}
