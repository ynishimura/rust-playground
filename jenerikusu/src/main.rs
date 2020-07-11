// &[i32] ：　i32値の具体的なスライスを示す
// fn largest(list: &[i32]) -> i32 {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest(&number_list);
//     // max number is {}
//     print!("The largest number is {}", result);
//     assert_eq!(result, 100);
// }

// 構造体定義

// struct Point<T> {
//     x: T,
//     y: T,
// }

// ＜T,U＞とするとなんでよくなる
struct Point<T, U> {
    x: T,
    y: U,
}

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// impl Point<i32> {
//     //  平方根を取る。斜辺の長さ
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

fn main() {
    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };
    // let p = Point { x: 5, y: 10 };
    // print!("p.x = {}", p.x());
    // print!("p.x = {}", p.distance_from_origin());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    print!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
