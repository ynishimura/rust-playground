struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    println!("Hello, world!");
    // dはcより先にドロップされるため下記出力となる
    // Dropping CustomSmartPointer with data `other stuff`!
    // Dropping CustomSmartPointer with data `my stuff`!
    let c = CustomSmartPointer {
        data: String::from("some date"),
    };
    // Rustは、 Dropトレイトのdropメソッドを手動で呼ばせてくれません
    // c.drop(); //エラーになる
    drop(c); // std::mem::drop関数を使うことでdropできる
    println!("CustomSmartPointer dropped befor the end of main.")

    // dropした後はエラーになる
    // let d = CustomSmartPointer {
    //     data: String::from("other stuff"),
    // };
    // println!("CustomSmartPointers created.");
}
