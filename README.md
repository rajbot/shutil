# shutil
Rust shell utility helper library

## Installing

```
cargo add shutil
```

## Using command pipelines in rust

`shutil::pipe()` makes it easy to execute command pipelines in rust.

For example, say you want to execute the following pipeline:

```bash
echo foo | rev | tr 'a-z' 'A-Z'
```

This will echo the string "foo", reverse it, and then change lowercase characters to
uppercase. The result will be the string "OOF". Here is the equivalent rust code:

```rust
use shutil::pipe;

fn main() {
    // Executes `echo "foo" | rev | tr "a-z" "A-Z"`
    let output = pipe(vec![
        vec!["echo", "foo"],
        vec!["rev"],
        vec!["tr", "a-z", "A-Z"],
    ]);

    // prints "OOF"
    println!("{}", output.unwrap());
}
```
