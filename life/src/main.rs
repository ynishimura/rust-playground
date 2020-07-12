fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    println!("Hello, world!");
    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("r: {}", r);

    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }

    println!("The longest string is {}", result);

    // 構造体定義のライフタイム注釈
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'"); // '.'が見つかりませんでした
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
