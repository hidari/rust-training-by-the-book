extern crate ch17_03_oo_design_patterns;
use ch17_03_oo_design_patterns::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
