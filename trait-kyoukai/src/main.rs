// Tのトレイト境界にPartialOrdを指定する必要があります
// 引数の値がCopyトレイトを実装しない型を含む可能性も出てきたので
// トレイト境界にCopyを追加
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
// Pair<T>は、
// 内部の型Tが比較を可能にする
// PartialOrdトレイトと出力を可能にするDisplayトレイトを実装している時のみ、
// cmp_displayメソッドを実装します。
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // トレイト境界をつかった実装を実行
    let p1 = Pair::new(10, 20);
    let p2 = Pair::new(-9.99, -20.0);
    p1.cmp_display();
    p2.cmp_display();

    // 比較できない型をいれてみる(Displayトレイトを持たない)
    let p3 = Pair::new(vec![1, 2, 3], vec![4, 5, 6]);
    p3.cmp_display();
}
