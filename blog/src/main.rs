use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("做你的太阳在你的心里");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("做你的太阳在你的心里", post.content());
}