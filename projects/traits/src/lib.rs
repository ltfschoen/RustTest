use std::fmt::Display;

// Media Aggregator

// TRAITS

/* Summary Trait for the media aggregator library that:
 * - Returns summary of data stored in NewsArticle or Tweet Type instance
 *   using a Summary from each Type that we call using a `summarize`
 *   method on the instance
 */
pub trait Summary {
    fn valid_word_count(&self) -> String;

    fn summarize_author(&self) -> String;

    // Replace semicolon with block and remove implementation 
    // to enable default trait
    fn summarize(&self) -> String; 
    // {
    //     format!("(Read more from {}...)", self.summarize_author());
    // }
}

// STRUCTS

pub struct NewsArticle<T> {
    pub headline: T,
    pub location: T,
    pub author: T,
    pub content: String
}

impl<T> Summary for NewsArticle<T> 
    where T: Display + PartialOrd
{
    // Comment out these methods and remove block proportion of the
    // Default Trait associated to revert back to using the Default Trait

    fn valid_word_count(&self) -> String {
        let content_word_count: usize = self.content.chars().count();
        let content_word_count_limit: usize = 50;
        if self.content.chars().count() < content_word_count_limit {
            format!("Good news article content count {} that complies with word count limit {}", 
                content_word_count,
                content_word_count_limit
            )
        } else {
            format!("Bad news article content count {} that exceeds word count limit {}", 
                content_word_count,
                content_word_count_limit
            )
        }
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({}), with {}", self.headline, self.summarize_author(), self.location, self.valid_word_count())
    }
}

pub struct Tweet<T> {
    pub username: T,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl<T> Summary for Tweet<T> 
    where T: Display + PartialOrd
{
    // Comment out these methods and remove block proportion of the
    // Default Trait associated to revert back to using the Default Trait

    fn valid_word_count(&self) -> String {
        let content_word_count: usize = self.content.chars().count();
        let content_word_count_limit: usize = 20;
        if self.content.chars().count() < content_word_count_limit {
            format!("Good tweet content count {} that complies with word count limit {}", 
                content_word_count,
                content_word_count_limit
            )
        } else {
            format!("Bad tweet content count {} that exceeds word count limit {}", 
                content_word_count,
                content_word_count_limit
            )
        }
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    
    fn summarize(&self) -> String {
        format!("{}: {}: {}:", self.summarize_author(), self.content, self.valid_word_count())
    }
}

/* Trait Bounds on type `T` declared with declaration of Generic Type Parameter
 * may be passed any instance of `NewsArticle` or `Tweet` in the main.rs implementation
 * since those types implement `Summary`
 */
pub fn notify<T>(item: T) 
    where T: Summary
{
    println!("Breaking news! {}", item.summarize());
}