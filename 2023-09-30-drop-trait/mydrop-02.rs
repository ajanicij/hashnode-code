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
  let mut ms = MyStruct { text: "hello".to_string(), val: 42 };
  {
      let mut ms2;
      println!("In then branch");
      ms2 = ms; // Move ms to ms2
      println!("After move");
      ms2.val = 43;
  }
  // ms.val = 1;
}

fn main() {
  println!("hello from mydrop");
  myfun();
}
