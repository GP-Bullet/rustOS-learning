//!trait 是一种抽象
//!相当于都具有某几个方法
//!trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。
//! 可以通过 trait 以一种抽象的方式定义共享的行为。可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。

pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
//为类型实现trait
//具体实现
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//实现trait的约束
//孤儿原则

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify(item: &(impl Summary + Display)) {
    
pub fn notify<T: Summary + Display>(item: &T) {
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    
    
    
//增加可读性
fn some_function<T, U>(t: &T, u: &U) -> i32
where 
T: Display + Clone,
        U: Clone + Debug
{

//!用trait作返回类型  所返回的同一种类型

