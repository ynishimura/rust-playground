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

    use std::collections::HashMap;
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores.entry(String::from("Yellow")).or_insert(20);
    scores.entry(String::from("Red")).or_insert(150); //なければ追加あれば無視
    scores.entry(String::from("Yellow")).or_insert(130); //なければ追加あれば無視

    // debug
    println!("{:?}", scores);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // こんな書き方もできる
    // let teams = vec![String::from("blue"), String::from("Yello")];
    // let initial_scores = vec![10, 50];
    // let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // 文字数カウント
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // 参照外し
    }
    println!("{:?}", map);
}
