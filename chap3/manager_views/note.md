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

1️⃣ Open pgAdmin - Open your browser and go to:
http://localhost:5050

2️⃣ Login to pgAdmin
Use the credentials from your compose file:
Email: admin@local.dev
Password: admin

🟢 Connection tab
Use container networking, not localhost:
Field	Value
Host name / address	postgres
Port	5432
Maintenance database	to_do
Username	username
Password	password
Save password	✅

Host name / address is `postgres` why not `localhost` ?
Because pgAdmin and Postgres are running in different containers.
Inside a container:
localhost always means “this same container”
So from pgAdmin container:
localhost ❌ → pgAdmin itself (no Postgres there)
postgres ✅ → Postgres container