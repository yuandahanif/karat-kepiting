pub mod blog {
    pub struct Post {
        content: String,
        state: Option<Box<dyn State>>,
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                content: String::from(""),
                state: Some(Box::new(Draft {})),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn content(&self) -> &str {
            self.state.as_ref().take().unwrap().content(self)
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review());
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve());
            }
        }
    }

    pub trait State {
        fn content<'a>(&self, _post: &'a Post) -> &'a str {
            ""
        }

        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn reject(self: Box<Self>) -> Box<dyn State>;
    }

    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview::new())
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    struct PendingReview {
        approved_count: u8,
    }

    impl PendingReview {
        pub fn new() -> PendingReview {
            PendingReview { approved_count: 0 }
        }
    }

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(mut self: Box<Self>) -> Box<dyn State> {
            if self.approved_count >= 1 {
                Box::new(Published {})
            } else {
                self.approved_count += 1;
                return self;
            }
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {})
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

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }
}

pub mod blog_2 {
    pub struct Post {
        content: String,
    }

    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: "".to_string(),
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }
    }

    pub struct DraftPost {
        content: String,
    }

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }

    pub struct PendingReviewPost {
        content: String,
    }

    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }
}
