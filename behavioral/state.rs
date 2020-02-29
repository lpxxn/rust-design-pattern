//! State is a behavioral design pattern that lets an object alter its behavior when its internal state changes.
//! It appears as if the object changed its class.

//! We’ll implement a blog post workflow
//! 1. A blog post starts as an empty draft.
//! 2. When the draft is done, a review of the post is requested.
//! 3. When the post is approved, it gets published.
//! 4. Only published blog posts return content to print, so unapproved posts can’t accidentally be published.

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft;
impl State for Draft {
    fn request_review(self: Box<Draft>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Draft>) -> Box<dyn State> {
        self
    }
}

struct PendingReview;
impl State for PendingReview {
    fn request_review(self: Box<PendingReview>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<PendingReview>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published;
impl State for Published {
    fn request_review(self: Box<Published>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Published>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

fn main() {
    let mut post = Post::new();

    let text = "State is a behavioral design pattern.";
    post.add_text(text);
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(text, post.content());
    println!("post content: {}", post.content());
}
