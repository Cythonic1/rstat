# rstat

> 🚀 **Rust-based alternative to Unix `stat` command**

A fast, safe Rust implementation of the classic `stat` tool to display file system metadata clearly and efficiently.

---

## 📋 Features

- Display detailed file and directory metadata (size, permissions, timestamps, etc.)
- Customizable output formatting
- Clean, structured CLI interface
- File type included

---

## 🛠️ Requirements

- Linux 👨‍⚖️
> **Note:** Windows support not yet implemented.

---

## 📦 Installation

```bash
git clone https://github.com/Cythonic1/rstat.git
cd rstat
cargo build --release
```

### Or install gloably
```bash
cargo install --path .
```

## Usage 🦀

```bash
rstat <FILE> [json, table]
```
 As the name says print it as table will print table json is json leaving it empty will print it as stat does.

## Future work. 📡
- Improve Error handling
- Make it more adaptive with other OS (win/mac)
