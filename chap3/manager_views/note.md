```rust [keywords]
use crate::to_do::to_do_factory; // use crate::  - use from /root (or /src)
use super::utils::return_state; // use super:: - use from /current  Dir ()
```

```bash
  echo 'actix-web = "2"\nactix-rt = "1.0"' >> Cargo.toml 
  cat Cargo.toml
  tree -I 'node_modules|dist|cache|target' tree cmd ignore

  # // ubuntu - services
  sudo systemctl stop postgresql
  sudo systemctl disable postgresql # Disable Auto-Start
  sudo systemctl status postgresql # check status

```

```bash
# Install diesel cli by: 
cargo install diesel_cli --no-default-features --features postgres

diesel setup # create migrations dir in the root
diesel migration run 
diesel migration redo 
diesel migration generate create_other_todo_items
```