## Test Server

This is test server for development, the server will be use to debug html render so every time we generate html highlight we do not need to reload page.

Start server:
```bash
cargo run --package test-server --bin hl-server
```

Generate debug html:
```bash
cargo run --package hl --example html examples/html.rs -l raw > table.html
```
