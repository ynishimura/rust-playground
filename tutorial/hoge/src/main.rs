#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}


fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue"))
    ];
    for i in &row {
        println!("{:?}", i);
    }
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);
    v.push(10);
    // let first = &v[0];

    // v.push(9);
    for i in &mut v {
        // *は参照外し演算子
        *i += 50;
        println!("{}",i);
    }
    // println!("{}", first);
    println!("Hello, world!");
}
