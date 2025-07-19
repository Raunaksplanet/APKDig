# APKDig

**APKDig** is a powerful tool designed to extract potentially sensitive files from Android APKs or already decompiled APK directories. It automates the process of locating and organizing critical files such as configuration files, database files, logs, and more — all useful during mobile application security assessments.

---

## 🔍 Features

* Supports both APK files and decompiled directories
* Automatically categorizes files based on extension
* Helps identify secrets, tokens, logs, backups, and more
* Useful for mobile bug bounty and penetration testing

---

## 📦 File Types Extracted

APKDig targets files with the following extensions:

* `.env`, `.properties`, `.conf`, `.cnf`, `.cfg`, `.ini`, `.yml`, `.yaml`
* `.db`, `.sqlite`, `.sqlite3`, `.pkl`, `.ndjson`, `.json`
* `.bak`, `.log`, `.swp`
* `.dex`, `.smali`, `.so`

---

## 🛠️ Requirements

* Python 3.x
* [apktool](https://github.com/iBotPeaches/Apktool) (only if using the `-a` option)

---

## 🚀 Usage

```bash
python apkdig.py -a myapp.apk
```

This will:

1. Decompile `myapp.apk` using apktool
2. Extract all targeted files into organized folders

Or if you already have a decompiled APK:

```bash
python apkdig.py -d path/to/decompiled_folder
```

---

## 📁 Output Structure

After execution, folders like `env_files/`, `json_files/`, `db_files/`, etc., will be created in the current directory, each containing the corresponding extracted files.

---

## 📌 Example

```bash
$ python apkdig.py -a sample.apk

[+] Decompiling APK...
[+] Extracting sensitive files...
├── env_files/
├── db_files/
├── json_files/
├── ...
```

---

## ⚠️ Disclaimer

This tool is intended for educational and authorized security testing purposes only. Unauthorized use is prohibited.

---

## 🧑‍💻 Author

**Raunak Gupta**
Founder of [Biscuit Security](https://github.com/biscuit-security)
[LinkedIn](https://linkedin.com/in/raunakgupta01) • [Medium](https://medium.com/@raunakgupta01)
