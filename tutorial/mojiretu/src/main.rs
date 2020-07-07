fn main() {
    // let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    // 同じ意味で書き方が違うだけ
    // let s =String::from("initial cotents");

    // let s = "initia; contents".to_string();
    let mut s1 = String::from("foo");
    let s2 = "boo";
    s1.push_str(s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", &s2);
    // 問題なく動く。push_strは所有権を得なくてもいいメソッド

    println!("Hello, world! {}", s);

    let sum = format!("{}-{}-{}", s, s1, s2);
    // format!は引数の所有権を奪わない
    println!("Hello, world! {}", sum);

    let hello = "3afffda";
    let s = &hello[0..4]; // 3aff
    println!("{}", s);

    for c in "プログラミング言語Rust".chars() {
        println!("{}", c);
    }
}
