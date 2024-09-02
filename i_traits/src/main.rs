// Lets say we have a program which shows different types of text content.
// In this case, a news article with fields author, headline and content.
// And A tweet with fields username, content, reply and retweet.

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// In this case, we want to summarize our news article and the tweet. We can use the trait to find the shared behavior between the two structs.
// Shared behaviour means methods.
// So in this case, summary is the trait.

pub trait Summary {
    fn summarize(&self) -> String;                      // Notice that we only specify the method signature and do not have the method body here.
                                                        // So we do not eictate the trait method but just that every type that implements this trait, will have a method called summarize and will return a string.
}
fn main() {
    let _a = 4;
}
