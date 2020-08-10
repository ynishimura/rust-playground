enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // 5と10を含むリストaを作ります。
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));

    // 3で始まるbと4で始まるcをつくる
    // 5と10を含む最初のaリストを含めるとエラー。
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
