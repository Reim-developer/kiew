# Kiew. A Rust Command Line Tool for Website Scraping

**This project is licensed under the [MIT LICENSE](LICENSE)**

## Table of contents
- [Quick Start](#quick-start)
- [Build Requirements](#build-requirements)
- [Sample Usage](#sample-usage)

# Quick Start:
#### Installation:
### Source installation:
> **Note: Please read our requirements to build at [Build Requirements](#build-requirements)**

```bash
git clone https://github.com/Reim-developer/kiew.git
cd kiew
cargo build --release --verbose
cd target/release/ && kiew --help
```

- Binary Installation:
- Install at [release page](https://github.com/Reim-developer/kiew/releases)

### Build Requirements:
- `Git ≥ 2.39.5`
- `Rustup ≥ 1.27.1`
- `Rustc ≥ 1.84.1`
- `Cargo ≥ 1.84.1`

### Sample Usage:
**Match and count elements on a webpage:**

```bash
kiew match -w https://example.com -e div
```

**Find and show all element property of website**
```bash
kiew find -w https://example.com -e div
```
