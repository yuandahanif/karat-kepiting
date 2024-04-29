use blog_oop::blog::Post;
use blog_oop::blog_2::Post as Post_2;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    println!("{:?}", post.content());

    post_2();
}

fn post_2() {
    let mut post = Post_2::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
    println!("{:?}", post.content());
}
