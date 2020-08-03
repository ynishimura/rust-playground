// 無限のサイズと判定されてコンパイルエラー
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    // let y = &x;
    // これでもコンパイルはOK。Box<T>を参照のように使える
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // これはエラー
    // assert_eq!(5, y);
}
