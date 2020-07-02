use std::io;

fn main() {
    println!("Guss the numbber!");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("fail");

    println!("you guessd: {}", guess);
}
