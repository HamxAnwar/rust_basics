// Lets say we have a program which shows different types of text content.
// In this case, a news article with fields author, headline and content.
// And A tweet with fields username, content, reply and retweet.

use std::fmt::Debug;
use std::fmt::Display;

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }

    // We have to implement this because there is no default implemented for summarize_author method.
    fn summarize_author(&self) -> String {
        format!("{:#?}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// For demonstration purposes of the default method, we comment out below.
impl Summary for Tweet {
    /*
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    */

    // We have to implement this because there is no default implemented for summarize_author method.
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

// In this case, we want to summarize our news article and the tweet. We can use the trait to find the shared behavior between the two structs.
// Shared behaviour means methods.
// So in this case, summary is the trait.

pub trait Summary {
    // fn summarize(&self) -> String; // Notice that we only specify the method signature and do not have the method body here.
    // So we do not dictate the trait method but just that every type that implements this trait, will have a method called summarize and will return a string.
    // We can also implement default methods. So we comment out the above and write the default. This method can be overwritten if the method is implemented in the impl blocks but if the implementation is not done, the default implementation will be used.

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more... from {})", self.summarize_author())
    }
}

// Traits can be used as parameters to a function.
// We have a new function called notify below. It takes in an item which is a reference to something which implements summary.
// So in the following function, item could be any type that implements summary trait.

/*
pub fn notify(item: &impl Summary) {
    eprintln!("Breaking news! {:#?}", item.summarize());
}
*/

// Following is the same function as above but it is just a more adaptive definition.
// Above could be used for more simple and concise cases.

pub fn notify<T: Summary>(item: &T) {
    eprintln!("Breaking news! {:#?}", item.summarize());
}

// We can also give multiple traits as parameters.
// So in the below examples, it says that item1 should be something that implements both Summary and Display traits.

pub fn notify_ex1(item1: &(impl Summary + Display), item2: &impl Summary) {}
pub fn notify_ex2<T: Summary + Display>(item1: &T, item2: &T) {}

// Below is the example to make code more readable.

fn example_fn1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}

// There is much gap between the parameters and the function name in the above example, so we can do instead:

fn example_fn2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    32
}

// Returning a trait type is very useful for closures and iterators.
// Restriction: If we implement here a function that returns a trait but it is implemented in a way such as it returns news article if something is true else return tweet, it is not possible. 
// We can also implement traits as return types as follows:

fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("Hamza Anwar"),
        content: String::from("This is the content."),
        reply: false,
        retweet: true,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("Hamza"),
        content: String::from("Rust is great language."),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("Hamza Anwar"),
        headline: String::from("This is the headline!"),
        content: String::from("This is the content of the article!"),
    };

    eprintln!("tweet = {:#?}", tweet.summarize());
    eprintln!("article = {:#?}", article.summarize());
    notify(&article);
    notify(&tweet);
    println!("Return summary = {}", return_summarizable().summarize());
}
