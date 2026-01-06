## Full stack RUST: completed guide
## handle error by Result<T, E>
struct Result {
  Ok(T),
  Err(E),
}

### Can check it bye use: `match` 
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt"); // Result<Ok(T), Err(E)>

    let greeting_file = match greeting_file_result {
        Ok(file) => file, // Ok(T)
        Err(error) => match error.kind() { // Err(E) - Result<Ok(T), Err(E)>
            ErrorKind::NotFound => match File::create("hello.txt") { // Ok(T) - Result<Ok(T), Err(E)>
                Ok(fc) => fc, Ok(T)
                Err(e) => panic!("Problem creating the file: {e:?}"), // Err(E)
            },
            _ => { // Err(E)
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}
```
### Reduce `match` by use closure |error| {} and `unwrap_or_else()` method
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = match File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {e:?}")
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}
```

### shortcut of panic on Error: 
.unwrap()
.expect("custom msg"): In production-quality code, most Rustaceans choose expect rather than unwrap and give more context about why the operation is expected to always succeed. 
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = match File::open("hello.txt").unwrap();
    let greeting_file_2 = match File::open("hello.txt").expect("cannot open file");
}
```

