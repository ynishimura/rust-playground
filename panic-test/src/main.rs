use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");

    // エラーをわざと発生させる
    // let v = vec![1, 2, 3];
    // v[99];

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
        },
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };
}
