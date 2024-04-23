use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summerize(&self) -> String;

    // default trait
    fn read_more(&self) -> String {
        // refferencing other trait
        format!("Read More from {}", self.summarize_author())
    }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summerize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Teweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Teweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summerize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// param must have trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summerize());
}

// all params must have same trait
pub fn is_same_author<T: Summary>(item: &T, item2: &T) -> bool {
    item.summarize_author() == item2.summarize_author()
}

// implement multiple trait
pub fn notify_log(item: &(impl Summary + Display)) {
    println!("{}", item);
    println!("Breaking news! {}", item.summerize());
}

// all params must have same trait
pub fn is_same_author_log<T: Summary + Display>(item: &T, item2: &T) -> bool {
    println!("{} and {}", item, item2);
    item.summarize_author() == item2.summarize_author()
}

// where
pub fn some_function<T, U>(item: &T, item2: &U) -> u32
where
    T: Display + Summary,
    U: Display,
{
    println!("{} {}", item, item2);
    32
}

//  must have trait to return
pub fn returns_summarizable() -> impl Summary {
    Teweet {
        username: todo!(),
        content: todo!(),
        reply: todo!(),
        retweet: todo!(),
    }
}
