// メモ
// <T>とかするのがジェネリック構文
// ジェネリック構文によって型の幅が広がる(型指定がなくなるため) そもそもジェネリックよくわかってなかった
// trait境界構文ってのがあるンゴ このtraitを持っていないと使えないことを明記できる

#![allow(unused)]

// トレイトを宣言(インターフェースみたいなもんだ)
pub trait Summary {
    fn summarize(&self) -> String;
}
pub trait Sunny {
    fn shine(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Sunny for NewsArticle {
    fn shine(&self) -> String {
        format!("sunshine: {}", self.author)
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

impl Sunny for Tweet {
    fn shine(&self) -> String {
        format!("sunshine: {}", self.username)
    }
}

// これはダメ
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// PartialOrdとCopyを持ってないといけなくする
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//ジェネリックとトレイトの話
//これで SunnyとSummary持たないといけない

//pub fn notify<T: Summary>(item: &T) {

// これがトレイト境界構文
pub fn notify<T: Sunny + Summary>(item: &T) {
    // 速報！ {}
    println!("Breaking news! {}", item.shine());
}

fn main() {
    let tweet = Tweet {
        username: String::from("You-saku"),
        content: String::from(
            "Rustのジェネリックとトレイト",
        ),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.summarize());
    let numbers = vec![1,4,5,10,2];
    let result = largest(&numbers);
    println!("Largest : {}", result);

    notify(&tweet);
}
