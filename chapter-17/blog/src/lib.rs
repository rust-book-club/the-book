pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    pub fn get_state(&self) -> &'static str {
        self.state.as_ref().unwrap().debug_state()
    }
}

trait State {
    fn debug_state(&self) -> &'static str;
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn debug_state(&self) -> &'static str {
        "Draft"
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { n_reviews: 0 })
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {
    n_reviews: u8
}

impl State for PendingReview {
    fn debug_state(&self) -> &'static str {
        "PendingReview"
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        if self.n_reviews > 0 {
            Box::new(Published {})
        } else {
            Box::new(PendingReview { n_reviews: 1 })
        }
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn debug_state(&self) -> &'static str {
        "Published"
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        &_post.content
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}