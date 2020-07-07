fn main() {
    // let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    // 同じ意味で書き方が違うだけ
    // let s =String::from("initial cotents");

    // let s = "initia; contents".to_string();
    println!("Hello, world! {}", s);
}
