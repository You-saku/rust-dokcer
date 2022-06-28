fn main() {
    // あれっ スタックしたものってどうなるんだっけ？
    let a = 10;
    let b = a;

    // これはヒープ (これは死ぬ)
    let s1 = String::from("Word");
    //let s2 = s1; このまま行くと borrowされちゃう
    let s2 = s1.clone();
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    println!("a = {}", a);
    println!("b = {}", b);

    let x = 5;
    let y = &x;

    println!("{}", x);
    println!("{}", *y); // ここら辺はCに似てるな
}
