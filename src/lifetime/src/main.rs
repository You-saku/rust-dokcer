#![allow(unused)]

// この状態だと xとyが渡されてもどっちを返せば良いかわからない
// xの場合もあるし、yの場合もある。しかし、どっちかはメモリが死ぬ。どっちが死ぬのかコンパイルはわからンゴ
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn lifetime_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = "abc";
    let s2 = "abcd";
    let result = lifetime_longest(s1, s2);
    println!("{}", result);
}
