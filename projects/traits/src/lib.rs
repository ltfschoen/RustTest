// Media Aggregator

// TRAITS

/* Summary Trait for the media aggregator library that:
 * - Returns summary of data stored in NewsArticle or Tweet Type instance
 *   using a Summary from each Type that we call using a `summarize`
 *   method on the instance
 */
pub trait Summary {
    fn summarize_author(&self) -> String;

    // Replace semicolon with block and remove implementation 
    // to enable default trait
    fn summarize(&self) -> String; 
    // {
    //     format!("(Read more from {}...)", self.summarize_author());
    // }
}

// STRUCTS

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // Comment out these methods and remove block proportion of the
    // Default Trait associated to revert back to using the Default Trait

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.summarize_author(), self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // Comment out these methods and remove block proportion of the
    // Default Trait associated to revert back to using the Default Trait

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    
    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
}