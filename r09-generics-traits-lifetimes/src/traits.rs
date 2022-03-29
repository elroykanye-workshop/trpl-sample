use std::fmt::{Debug, Display, format};
use std::iter::Sum;

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {}

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

// ex 06
struct Pair<T> {
    x: T, y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y, }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y { println!("The largest member is x = {}", self.x) }
        else { println!("The largest member is = {}", self.y) }
    }
}

pub fn ex01() {
    let tweet = Tweet {
        username: String::from("elroykanye"),
        content: String::from("Of course, as you probably already know, people."),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: `{}`", tweet.summarize());
}

pub fn ex02() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    println!("1 new article: `{}`", article.summarize());
}

pub fn ex03_notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize())
}

pub fn ex04_some_fn<T, U> (t: T, u: U) -> i32 where T: Clone + Debug, U: Clone + Debug {
    0
}

pub fn ex05_1_generics() {
    let num_list = vec![34, 50, 25, 100, 65];

    let res = ex05_2_largest(&num_list);
    println!("The largest number is {}", res);

    let char_list = vec!['e', 'l', 'r', 'o', 'y'];
    let res = ex05_2_largest(&char_list);
    println!("The largest character is {}", res);

}

fn ex05_2_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest { largest = item; }
    }
    largest
}

fn ex06() {

}