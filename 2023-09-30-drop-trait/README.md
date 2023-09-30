## Drop trait and Rust ownership

This is something about Rust I find fascinating. It is about how the
compiler manages to do the right thing. I'll explain what I mean, but first
we have to prepare the scene.

We will create a structure and implement Drop trait for it.

Here's the structure:

```
#[derive(Debug)]
struct MyStruct {
  text: String,
  val: i32,
}
```

And here's the Drop trait:

```
impl Drop for MyStruct {
  fn drop(&mut self) {
    println!("In MyStruct::drop: text={} val={}", self.text, self.val);
  }
}
```

Here, we just want to track when the drop is called. Before an object is
disposed, if it is an instance of a struct for which Drop is implemented,
drop method has to be called.

Our first function is the simplest: we just declare an instance of MyStruct
so we can see when drop is called:

```
fn myfun() {
  println!("In myfun");
  let ms = MyStruct { text: "hello".to_string(), val: 42 };
  println!("Created an instance of MyStruct: {:?}", ms);
}
```

When we call myfun, here's what the output looks like:

```
In myfun
Created an instance of MyStruct: MyStruct { text: "hello", val: 42 }
In MyStruct::drop: text=hello val=42
```


## Move

The second step is to move an object from one variable to another. First,
a version that doesn't compile:

```
fn myfun() {
  println!("In myfun");
  let mut ms = MyStruct { text: "hello".to_string(), val: 42 };
  {
      let mut ms2: MyStruct;
      println!("In then branch");
      ms2 = ms; // Move ms to ms2
      ms2.val = 43;
  }
  ms.val = 1;
}
```

Compiler doesn't like this:

```
error[E0382]: assign of moved value: `ms`
  --> mydrop-02.rs:22:3
   |
15 |   let mut ms = MyStruct { text: "hello".to_string(), val: 42 };
   |       ------ move occurs because `ms` has type `MyStruct`, which does not implement the `Copy` trait
...
19 |       ms2 = ms; // Move ms to ms2
   |             -- value moved here
...
22 |   ms.val = 1;
   |   ^^^^^^^^^^ value assigned here after move
```

That is another fascinating thing about Rust: in many cases the error message is
describing exactly what the compiler doesn't like. Here we declared an instance ms of
MyStruct, then we declared another, ms2. After executing ms2 = ms, the object has
been moved so now it is owned by ms2. (I we had assigned an object to ms2 before
this ownership transfer, that other object would be dropped by executing ms2 = ms,
and we would see drop function being called.) At this point, ms owns nothing, so
ms.var = 1; makes no sense and the compiler does not like it.

OK, so we comment out that assignment:

```
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
```

and now the program compiles (but complains that ms doesn't have to be declared
as mutable; ignore that). The output is:

```
In myfun
In then branch
After move
In MyStruct::drop: text=hello val=43
```

## Possible move

OK, now for the sweet part. What if I write code in such a way that the compiler
doesn't know exactly where some object has to be dropped?

```
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
```

See what we did here? Depending on the value of variable x, a move from ms to ms2
may or may not be done in the inner block of the function. Then at the end of the
function, ms is assigned or unassigned. We try both with this main function:

```
fn main() {
  println!("hello from mydrop");
  myfun(0);
  println!("------------");
  myfun(1);
}
```

And sure enough, it all works:

```
hello from mydrop
in myfun: x=0
In then branch
In MyStruct::drop: text=hello val=43
After then branch
------------
in myfun: x=1
After then branch
In MyStruct::drop: text=hello val=42
```

If x=0, the object is dropped just before the end of the inner block, when
ms2 goes out of scope. Otherwise, there is no move and the object is dropped
later, when ms goes out of scope just before myfun returns.

That is what I find fascinating. How does the code know whether to drop or not?
I have not analyzed the compiled code, but I recon there has to be some hidden
variable created by the compiler that keeps track of whether ms is assigned or
not.
