extern crate traits;

use traits::{Summary, notify};

fn main() {
    let tweet = traits::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let news_article = traits::NewsArticle {
        headline: String::from("Ethereum knifed"),
        location: String::from("Sydney"),
        author: String::from("Luke Schoen"),
        content: String::from("Ethereum was knifed by an EOS spoon")
    };
    println!("1 new news article: {}", news_article.summarize());

    // Trait Bounds implementations
    notify(tweet);
    notify(news_article);
}
