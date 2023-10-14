In this post, we work on the code that reads the contents of a directory.
First, we get the path of the directory from a command line argument:

```
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: readdir <dir-name>");
        exit(0);
    }
    let dir = &args[1];
```

Note that we are not using clap to get command line arguments. That is
because this program is very simple and has only one argument, the
directory that it has to read and clap would be an overkill. But in the
final program we will use clap.

Next, we check if dir is really a directory by getting the metadata and
calling method is_dir. We will not show that code here, because we have
already done that in the previous post, but you can see the entire code
in the GitHub repository.

We get an iterator that is used for getting all entries in the directory
by calling fs::read_dir(dir):

```
    let pathlist = fs::read_dir(dir)?;
    let mut entries = Vec::new();
    for entry in pathlist {
        let entry = entry?;
        let file_name = extract_str_from_dir_entry(&entry)?;
        // We will push file_name to a vector here.
    }
```

This will list all the entries, but not necessarily in any order. For the
purposes of this post, we want to sort the entries alphabetically (simply
because ls command does that).

Function extract_str_from_dir_entry is as follows:

```
fn extract_str_from_dir_entry(entry: &DirEntry) -> Result<String, Error> {
    let path = entry.path();
    let file_name = match path.file_name() {
        Some(file_name) => file_name,
        None => return Err(Error::new(ErrorKind::Other, "bad path name"))
    };
    let file_name = match file_name.to_str() {
        Some(file_name) => file_name,
        None => return Err(Error::new(ErrorKind::Other, "path name not a string"))
    };
    Ok(file_name.to_owned())
}
```

This looks complicated for the simple task of getting file name, so let's
unpack it.

entry argument has the type
[DirEntry](https://doc.rust-lang.org/std/fs/struct.DirEntry.html)

To get the full path of the entry, we use method
[path](https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.path).
It returns
[PathBuf](https://doc.rust-lang.org/std/path/struct.PathBuf.html).

To extract the file name from the path, we use method
[file_name](https://doc.rust-lang.org/std/path/struct.PathBuf.html#method.file_name),
which returns
[OsStr](https://doc.rust-lang.org/std/ffi/struct.OsStr.html), which is similar
to &str. It a "borrowed reference to an OS string." Why doesn't file_name() return
&str? Because Rust guarantees that &str and String always represent correct
UTF-8 strings, and a file path in various OSs is not guaranteed to be a correct
UTF-8 string.

To convert file name from OsStr to &str, we use method
[to_str](https://doc.rust-lang.org/std/ffi/struct.OsStr.html#method.to_str), which
returns Option<&str>. If the file name is not a proper UTF-8 string, to_str() will
return None. That's the beauty of Rust: it uses the type system to ensure that
you won't forget to deal with stuff like this.

We want our function extract_str_from_dir_entry to return a String, not &str
(actually, it returns Result<String, Error>, because it can fail, as we have
just explained). To convert from &str to an owned string, we use to_owned.
to_string would have worked too, but to_owned is
[more efficient](https://users.rust-lang.org/t/to-string-vs-to-owned-for-string-literals/1441).

## Sorting directory entries

One final thing to discuss is sorting the directory entries. I checked how ls
works: it sorts the entries in case-insensitive way, so that is what I
implemented here.

```
    let mut entries = Vec::new();
    for entry in pathlist {
        let entry = entry?;
        let file_name = extract_str_from_dir_entry(&entry)?;
        entries.push(file_name);
    }
    println!("Sorted:");
    entries.sort_by(|a, b| a.to_lowercase().partial_cmp(&b.to_lowercase()).unwrap());
    for entry in entries {
        println!("{}", entry);
    }
```

First we declare a vector of strings for entries and as we iterate through the
entries, we push them into this vector. After that, we sort the vector, passing
the closure that converts strings to lowercase before comparing them.

And that's it.

