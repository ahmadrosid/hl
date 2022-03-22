## Test Server

We use this server to debug HTML rendering so every time we generate a html highlight we won't have to reload the page.

Start server:
```bash
cargo run --package test-server --bin hl-server
```

Generate debug html:
```bash
cargo run --package hl --example html examples/html.rs -l raw > table.html
```

Open webserver at [http://localhost:8080](http://localhost:8080)
