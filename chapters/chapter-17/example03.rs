
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

    pub fn approve(&mut self) {
	if let Some(s) = self.state.take() {
	    self.state = Some(s.approve())
	}
    }

    pub fn content(&self) -> &str {
	&self.content
    }

    pub fn request_review(&mut self) {
	if let Some(s) = self.state.take() {
	    self.state = Some(s.request_review())
	}
    }
}

struct Draft {}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
	self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
	self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
	self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
	Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
	self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
	self
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
