use std::arch::x86_64;

fn main() {
    println!("Hello, world!");
    let tweet = Tweet {
        username:String::from("horse_ebooks"),
        content:String::from("of course, as you probably already know, people"),
        reply:false,
        retweet:false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    // println!("{}", article.summarize());
    notify(&article); 
 

}


pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.content)
    }
    
}
impl Summary for Tweet{
    fn summarize(&self) -> String {
        format!(" by{}({})",self.username,self.content) 

    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}


pub fn notify(item:&impl Summary){
    println!("Breaking news! {}", item.summarize());
}
pub fn notify2<T:Summary>(item:&T){
    println!("Breaking news! {}", item.summarize());
}

// generic lifetimes
pub fn longest<'a>(x:&'a str,y:&'a str) -> &'a str{
    if x.len() > y.len(){
        x
    }else{
        y
    }
}