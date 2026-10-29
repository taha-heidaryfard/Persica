<div align="center">

# 🌿 Persica Programming Language

**A lightweight, statically-typed, and developer-friendly programming language built from scratch in Rust.**

[![Version](https://img.shields.io/badge/version-0.13.0-blue.svg?style=for-the-badge)](https://github.com/taha-heidaryfard/persica/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](LICENSE)
[![Tests Passing](https://img.shields.io/badge/tests-138%20passed-success.svg?style=for-the-badge)]()
[![Zero Dependencies](https://img.shields.io/badge/dependencies-0%20crates-orange.svg?style=for-the-badge)]()
[![Donate](https://img.shields.io/badge/Donate-Reymit-ff5252.svg?style=for-the-badge&logo=heart)](https://reymit.ir/taxtander)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg?style=for-the-badge)]()

[English](#english) • [فارسی](#persian) • [Documentation Website](https://taha-heidaryfard.github.io/persica/) • [Releases](https://github.com/taha-heidaryfard/persica/releases) • [💖 Donate / حمایت](https://reymit.ir/taxtander)

---

</div>

<a name="english"></a>
## 🌟 About Persica

**Persica** combines the **ergonomics and syntactic beauty of Python** with the **safety and predictability of a statically-typed language**, powered by the raw speed and zero-dependency reliability of Rust.

Every layer of the compiler pipeline—Lexer, Recursive-Descent Parser, Abstract Syntax Tree (AST), Semantic Type Checker, Tree-Walk Interpreter, and Project Manager—is crafted completely from scratch without third-party crates.

### ✨ Key Highlights

- **⚡ Zero Ceremony Execution:** Run individual scripts instantly like Python (`persica script.persi`) without project scaffolding.
- **📁 Built-in Project System:** Configure multi-module projects with `persica.toml` (`persica new`, `persica run`, `persica test`).
- **🔒 Isolated `.persi_env` Environment:** Isolated local package environment for third-party modules without duplicating heavy executable binaries.
- **🛡️ Full Exception Handling:** Robust `try / catch / throw` syntax with call-stack unwinding and runtime error trapping (Zero Rust panics!).
- **🏛️ Classes & Structs:** Pure-data value structs (`struct`) and rich object-oriented classes with methods and reference semantics (`class`).
- **📦 Python-style Modular Imports:** Clean imports without quotes (`import math as m;`, `from models import Point, *;`).
- **📚 Self-Hosted Standard Library & Auto-Prelude:** Standard data structures (`Stack`, `Queue`, `Set`, `StringBuilder`, `Pair`, `Result`, `Option`) ready out-of-the-box.
- **🪟 Windows Setup Installer:** Official 1-click Windows installer with automatic `PATH` configuration and `.persi` file association.

---

## 🚀 Quick Start

### 1. Installation

#### Windows (Official Installer)
Download the latest `Persica-0.13.0-Windows-x64-Setup.exe` from [Releases](https://github.com/taha-heidaryfard/persica/releases) and run the installer. It configures `persica` in your system `PATH` automatically.

#### Build from Source (Cargo)
```bash
git clone https://github.com/taha-heidaryfard/persica.git
cd persica
cargo build --release
```
The compiled binary will be located in `target/release/persica.exe`.

---

### 2. Running Code

#### Instant Single-File Run
Create `hello.persi`:
```persica
print("Hello from Persica!");

for i in range(1, 5) {
    print("Step:", i);
}
```

Run it immediately:
```bash
persica hello.persi
```

#### Starting a Full Project
```bash
# Create a new project
persica new my_app
cd my_app

# Run the project
persica run

# Run project unit tests
persica test
```

A generated `persica.toml` looks like:
```toml
[package]
name = "my_app"
version = "0.1.0"
entry = "src/main.persi"
source = "src"

[environment]
env_dir = ".persi_env"
```

---

## 💻 Language Tour

### Variables & Static Types
```persica
int age = 25;
float pi = 3.14159;
string name = "Persica";
bool active = true;

// Final immutable binding
final string version = "0.13.0";
```

### Classes and Structs
```persica
// Value-semantics pure data structure
struct Point {
    int x;
    int y;
}

// Reference-semantics object-oriented class
class Counter {
    int value = 0;

    void increment() {
        this.value = this.value + 1;
    }

    int get_count() {
        return this.value;
    }
}

Point pt = Point(10, 20);
Counter c = Counter();
c.increment();
print("Count is:", c.get_count());
```

### Exception Handling (`try / catch / throw`)
```persica
try {
    int divisor = 0;
    if divisor == 0 {
        throw "Cannot divide by zero!";
    }
} catch (error) {
    print("Caught an exception:", error);
}
```

### Modular Imports & Prelude
```persica
// Auto-Prelude data structures are available everywhere
Stack stack = Stack();
stack.push("Alpha");
stack.push("Beta");
print("Top item:", stack.pop());

// Import standard or custom modules
from std.math import sqrt, pow;
print("Square root of 16 is:", sqrt(16.0));
```

---

<a name="persian"></a>
## 🇮🇷 درباره پرسیکا (Persica)

**پرسیکا** یک زبان برنامه‌نویسی مدرن، ایستا-نوع (Statically-Typed) و مستقل است که با هدف ارائه‌ی سینتکسی تمیز و خوانا (مشابه پایتون) در کنار ایمنی، سرعت بالا و عدم وابستگی به هیچ کتابخانه جانبی در زبان Rust توسعه یافته است.

### امکانات برجسته:
- **اجرای آسان تک‌فایل:** فایل‌های `.persi` را همانند پایتون بدون نیاز به ساخت پوشه یا پروژه اجرا کنید.
- **سیستم مدیریت پروژه و محیط ایزوله:** دارای ابزارهای `persica new`, `persica run`, `persica test` همراه با کانفیگ سبک `persica.toml` و محیط محلی ایزوله ماژول‌ها بنام `.persi_env`.
- **مدل شیء‌گرایی و ساختارهای داده:** پشتیبانی از مدل داده‌ای خالص `struct` و کلاس‌های شیء‌گرا دارای رفتار و متد `class`.
- **سیستم استثنا و مدیریت خطا:** بلوک‌های `try / catch / throw` با خروج امن پشته و بدون کوچک‌ترین Panic در رانتایم زبان.
- **کتابخانه استاندارد خود‌میزبان:** پیاده‌سازی ساختارهای داده پرکاربرد (`Stack`, `Queue`, `Set`, `StringBuilder`) و الگوریتم‌های استاندارد با خود زبان پرسیکا.
- **نصاب اختصاصی ویندوز:** فایل راه‌انداز رسمی برای ویندوز ۶۴ بیتی با تنظیم خودکار متغیرهای محیطی `PATH`.

---

## 🛠️ CLI Reference

| دستور | شرح |
|---|---|
| `persica <file.persi>` | اجرای مستقیم اسکریپت تکی |
| `persica new <name>` | ساخت پروژه جدید با اسکلت‌بندی کامل |
| `persica init` | مقداردهی اولیه پوشه فعلی به عنوان پروژه |
| `persica run` | اجرای فایل اصلی پروژه طبق `persica.toml` |
| `persica test` | اجرای خودکار تمام تست‌های درون پوشه `tests/` |
| `persica check <file>` | تحلیل معنایی و نوع‌سنجی استاتیک بدون اجرا |
| `persica --eval "code"` | ارزیابی و اجرای قطعه کد به‌صورت درون‌خطی |

---

## 💖 Sponsorship & Support / حمایت مالی

Persica is a passion project built completely from scratch. If you find Persica interesting, useful, or want to support its ongoing research and development, you can support Taxtander via **Reymit**:

اگر زبان پرسیکا برای شما کاربردی بوده یا به توسعه زبان‌های برنامه‌نویسی علاقه‌مندید، با حمایت مالی خود به ادامه‌ی این مسیر کمک کنید:

<div align="center">

[![Donate via Reymit](https://img.shields.io/badge/حمایت_مالی-Reymit-ff5252.svg?style=for-the-badge&logo=heart)](https://reymit.ir/taxtander)

👉 **[رِیمیت | درگاه حمایت مالی از Taxtander](https://reymit.ir/taxtander)**

</div>

---

## 📄 License

This project is licensed under the terms of the [MIT License](LICENSE).  
Copyright (c) 2026 **Taha Heidary**.
