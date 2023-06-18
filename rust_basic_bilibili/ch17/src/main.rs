use ch17::*;

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Bottom {
                width: 10,
                height: 20,
                lable: String::from("bottom_1"),
            }),
            Box::new(Circle {
                radius: 3,
                lable: "radius_3".to_string(),
            }),
            Box::new(Bottom {
                width: 20,
                height: 10,
                lable: "bottom_2".to_string(),
            }),
        ],
    };

    screen.run();

    let mut post = Post::new();

    post.add_text("cool, I created a posting");

    let post = post.request_review();
    let post = post.approve();

    assert_eq!("cool, I created a posting", post.content());
}

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        /*
            requesting...
        */
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
        /*
            approving...
        */
        Post {
            content: self.content,
        }
    }
}
