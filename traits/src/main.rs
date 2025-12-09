pub struct NewsArticle {
    pub author: String,
    pub headline: String, 
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


pub struct Message {
    author: String,
    recipient: String,
    read: bool
}

impl Summary for Message {}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news {}", item.summarize())
}

pub fn notify_two<T: Summary, U: Summary> (item1: &T, item2: &U) {
    println!("Breaking news 1 {}\nBreaking news 2 {}", item1.summarize(), item2.summarize());
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more")
    }
}


fn main() {
    let tweet = Tweet {
        username: String::from("James"),
        content: String::from("I am learning rust!"),
        reply: false,
        retweet: false
    };

    let news_article = NewsArticle {
        author: String::from("Mich"),
        headline: String::from("Man wrestles with whale"),
        content: String::from("In North Carolina, a man walked up to a bear and just started fighting with it... He won tho")
    };

    println!("Summarised tweet: {}", tweet.summarize());
    println!("Summarised news_article: {}", news_article.summarize());

    let message = Message {
        author: String::from("Freeman"),
        recipient: String::from("Isaac"),
        read: true
    };

    println!("Summarised message: {}", message.summarize());

    notify(&news_article);
    println!("=========");
    notify_two(&news_article, &tweet);
}
