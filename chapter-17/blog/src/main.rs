use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("Draft", post.get_state());
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("PendingReview", post.get_state());
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("Draft", post.get_state());

    post.request_review();
    assert_eq!("PendingReview", post.get_state());
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("PendingReview", post.get_state());
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Published", post.get_state());
    assert_eq!("I ate a salad for lunch today", post.content());
}