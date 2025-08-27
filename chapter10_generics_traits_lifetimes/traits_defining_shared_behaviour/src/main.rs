fn main() {
    println!("Hello, world!");
}

pub trait Summary {
    fn summarise_content(&self) -> String;

    fn summarise(&self) -> String {
        self.summarise_content()
    }
}

pub struct NewsArticle {
    headline: String,
    author: String,
    content: String,
}

pub struct SocialMediaPost {
    likes: i32,
    user: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarise_content(&self) -> String {
        return self.content.clone();
    }
}

pub fn notify(item: &(impl Summary + MyTrait)) {
    println!("{}", item.summarise());
}

pub trait MyTrait {
    fn random_function() {}
}
