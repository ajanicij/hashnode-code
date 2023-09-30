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

fn myfun(x: i32) {
  println!("in myfun: x={}", x);
  let ms = MyStruct { text: "hello".to_string(), val: 42 };
  {
    if x == 0 {
      let mut ms2: MyStruct;
      println!("In then branch");
      ms2 = ms; // Move ms to ms2
      ms2.val = 43;
    }
    println!("After then branch");
  }
}

fn main() {
  println!("hello from mydrop");
  myfun(0);
  println!("------------");
  myfun(1);
}
