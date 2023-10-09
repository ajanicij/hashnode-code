The topic of today's post is opening and reading text files. Simple
stuff, and something that you can find described on countless pages,
including Rust's official documentation. I am doing this one for my
record.

## Reading the whole file with one function call

The first program:

- gets file name from the command line
- reads file contents into a string by calling read_to_string
- writes the contents to the standard output

```
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: read_file <input-file>");
        exit(0);
    }

    let file_path = &args[1];
    let s = fs::read_to_string(file_path).unwrap();
    println!("File contents:\n{}", s);
}
```

## Reading file line by line

For this, we:

- open file
- create a reader, an instance of BufReader
- from the reader, get an iterator to the lines
- iterate through all the lines and write them to the standard output

```
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: read_file <input-file>");
        exit(0);
    }

    let file_path = &args[1];
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);
    let lines = reader.lines();
    for line in lines {
        let s = line.unwrap();
        println!("Got line: {}", s);
    }
}
```

## Reporting error if the file cannot be opened

The only difference from the previous step is that now we don't call
unwrap():

    let file = File::open(file_path).unwrap();

If file opening fails for whatever reason, the program panics and
displays something like this:

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', textfiles-02.rs:16:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

This is OK for learning and practicing, but not so for production code.
unwrap(), just like expect(), is something that should only be called when
we expect the call to succeed. In other words, if this causes panic, then
we have a bug.

In case of opening a file, if a file is not found or our program is not
authorized to open it, it should not panic but rather display an error
message and exit (or not, depending on what we want to do in the program).

For example, instead of calling unwrap, we can do this:

```
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening {}: {}", file_path, e.to_string());
            std::process::exit(1);
        }
    };
```

That code behaves much more decently, and if the file is not found, writes
an error message:

    Error opening input.txt2: No such file or directory (os error 2)

and exits with code 1 (any code other than 0 means there was an error).

Get the code at

[https://github.com/ajanicij/hashnode-code/tree/master/2023-10-02-text-files](https://github.com/ajanicij/hashnode-code/tree/master/2023-10-02-text-files)

