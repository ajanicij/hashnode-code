In this installment on our road to implementing a mini version of ls
in Rust, we are going to get information about one file:

- Is it a regular file or a directory?
- File permissions
- Time of last modification
- Number of hard links

That is the kind of information that

    ls -l

gets. For example, when I run it in the project root:

    ls -l | grep src

I get the following:

    drwxrwxr-x 2 aleks aleks  4096 Oct 10 20:41 src

which gives us the information about directory src:

- drwxrwxr-x
  - it is a directory.
  - User permissions: read, write, execute (it is always x for a directory)
  - Group permissions: read, write, execute
  - "Others" permissions: read, execute (others don't have a write permission)
- 2 - the number of hard links
- aleks:aleks - user and group
- 4096 - size
- Oct 10 20:41 - date/time of last modification
- src - name of the directory

We can get all this information from file's metadata.

## Metadata

We get metadata by executing:

    let metadata = fs::metadata(file)?;

We can check it file is a directory:

    let is_dir = metadata.is_dir();

To check if it is a regular file:

    let is_file = metadata.is_file();

To get file permissions:

```
let permissions = metadata.permissions();
let mode = permissions.mode();
```

mode contains all flags for file permissions. They are defined in
[crate libc](https://docs.rs/libc/latest/libc/).
For example, to check if the file has user-read permission, we
can check if

    mode & libc::S_IRUSR

is nonzero.

To get the time of last modification, we can use
[modified](https://doc.rust-lang.org/std/fs/struct.Metadata.html#method.modified).
That method returns SystemTime, hut to display it, we need to convert it
to DateTime. We could first convert it to the number of seconds since the
epoch, and then to DateTime, but after some search I found crate
[systime_converter](https://lib.rs/crates/systime_converter), which does
exactly that.

Before we perform the conversion, we need one more piece of information:
the local time zone. Here's how we get that:

```
    let now = Local::now();
    let tz = now.timezone();
```

Now we convert to DateTime:

```
    let datetime = systime_converter::convert(modified, tz);
    println!("strftime: {}", datetime.format("%Y %b %e %H:%M"));
```

And one last thing we need to know about the file is the number of hard
links:

    metadata.st_nlink()

Get the code at

[2023-10-10-metadata](https://github.com/ajanicij/hashnode-code/tree/master/2023-10-10-metadata/filestats)
