# Developer documentation for HandyDB

---

## Release

### Add targets (docker required)

```bash
rustup target add x86_64-unknown-linux-gnu
```

```bash
rustup target add x86_64-pc-windows-msvc
```

```bash
rustup target add aarch64_unknown-linux-gnu
```

### Build releases

```bash
cargo build --release
```

```bash
cross build --target x86_64-pc-windows-gnu --release
```

### Archive releases

```bash
tar -czvf no-sql-rust-database-linux.tar.gz -C target/release no-sql-rust-database
```

```bash
zip no-sql-rust-database-windows.zip -j target/x86_64-pc-windows-gnu/release/no-sql-rust-database.exe
```

---

## Configuration

### Installing Rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installing cross (optional)

```bash
cargo install cross
```

---

## Possible bugfixes

### Windows

#### If the bash shell isn't working

1. Create ~/.bashrc file:

```bash
touch ~/.bashrc
```

1. Add PATH export to it:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

1. Reload:

```bash
source ~/.bashrc
```

1. Verify everything is here:

```bash
rustc --version
cargo --version
```
