# APKDig

APKDig is a Rust-based utility that extracts sensitive or high-value files from Android APKs or already-decompiled APK directories. It automates the discovery and collection of configuration files, databases, logs, backups, and other artifacts useful during mobile security testing.

---

## Features

* Works on APK files and decompiled directories.
* Automatically categorizes extracted files by type.
* Detects configs, databases, logs, backups, smali, native libs, and more.
* Helpful for mobile bug bounty, forensics, and pentesting workflows.

---

## File Types Extracted

Targeted extensions include:

* Config: `.env`, `.properties`, `.conf`, `.cnf`, `.cfg`, `.ini`, `.yml`, `.yaml`
* Databases: `.db`, `.sqlite`, `.sqlite3`, `.pkl`, `.ndjson`, `.json`
* Backups / Logs / Misc: `.bak`, `.log`, `.swp`
* Code / Artifacts: `.dex`, `.smali`, `.so`

---

## Installation

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
```

(optional but recommended)

```bash
echo '. "$HOME/.cargo/env"' >> ~/.zshrc
```

### Build APKDig

```bash
cd APKDig-main
cargo build --release
```

The compiled binary will be available at:

```
target/release/apkdig
```

---

## Usage

### Run against an APK

```bash
./target/release/apkdig -a myapp.apk
```

This will:

1. Decompile `myapp.apk` (if the tool supports decompiling).
2. Scan the directory.
3. Extract matched files into organized output folders.

### Run against an already-decompiled directory

```bash
./target/release/apkdig -d /path/to/decompiled_apk/
```

### View all options

```bash
./target/release/apkdig --help
```

---

## Output Structure

The tool creates categorized folders such as:

```
env_files/
db_files/
json_files/
dex_files/
smali_files/
so_files/
log_files/
...
```

Each folder contains the extracted files relevant to that category.

---

## Example

```bash
$ ./target/release/apkdig -a sample.apk

[+] Decompiling...
[+] Scanning for sensitive files...
[+] Extracted:
├── env_files/
├── db_files/
├── json_files/
├── ...
```

---

## Disclaimer

Use this tool only for authorized security testing and research. Any unauthorized use is prohibited.

---

## Author

Raunak Gupta
Founder, Biscuit Security
LinkedIn: [https://linkedin.com/in/raunakgupta01](https://linkedin.com/in/raunakgupta01)
Portfolio: [https://www.b1scuit.pro/](https://www.b1scuit.pro/)
