// &[i32] ：　i32値の具体的なスライスを示す
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    // max number is {}
    print!("The largest number is {}", result);
    assert_eq!(result, 100);
}
