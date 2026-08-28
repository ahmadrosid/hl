## Test Server

Live-reloads the browser when `table.html` changes. Zero deps — plain `std` HTTP + mtime poll.

Start server:
```bash
cargo run --package test-server --bin hl-server
```

Generate debug html:
```bash
cargo run --package hl --example html examples/html.rs -l raw > table.html
```

Open [http://127.0.0.1:8080](http://127.0.0.1:8080)
