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
tar -czvf handy_db-x86_64-linux-gnu.tar.gz -C target/release handy_db
```

```bash
zip handy_db-x86_64-windows-gnu.zip -j target/x86_64-pc-windows-gnu/release/handy_db.exe
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
