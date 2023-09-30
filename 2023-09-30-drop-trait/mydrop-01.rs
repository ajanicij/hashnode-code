#[derive(Debug)]
struct MyStruct {
    text: String,
    val: i32,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("In MyStruct::drop: text={} val={}", self.text, self.val);
    }
}

fn myfun() {
    println!("In myfun");
    let ms = MyStruct {
        text: "hello".to_string(),
        val: 42,
    };
    println!("Created an instance of MyStruct: {:?}", ms);
}

fn main() {
    println!("hello from mydrop");
    myfun();
}
