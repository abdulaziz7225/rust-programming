use oop_design_pattern::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    // assert_eq!("", post.content());
    
    // post.request_review();
    // assert_eq!("", post.content());
    let post = post.request_review();
    
    // post.approve();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
