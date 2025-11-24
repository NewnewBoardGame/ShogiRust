### Shogi

### How to run this project

#### Go
Building and running `shogi_go`:

```bash
go build shogi_go/shogi_go.go && ./shogi_go/shogi_go
```

#### Rust
Building and running `shogi_rust`:

```bash
cargo +nightly -C shogi_rust build -Z unstable-options && ./shogi_rust/target/debug/shogi_rust
```

or if that does not work due to the Nightly `-C`:
```bash
cd shogi_rust && cargo build && ./target/debug/shogi_rust
```