# نصب، build و اجرای Persica

## پیش‌نیاز

Persica به Rust stable با پشتیبانی از edition 2024 نیاز دارد. پروژه dependency خارجی ندارد.

بررسی نصب:

```powershell
rustc --version
cargo --version
```

در سیستم فعلی ابزارها در مسیر زیر نصب شده‌اند؛ اگر `cargo` در PATH نبود می‌توان مسیر کامل را استفاده کرد:

```powershell
C:\Users\Taxtander\.cargo\bin\cargo.exe --version
```

## اجرای مستقیم از source

از ریشه‌ی پروژه:

```powershell
cargo run
cargo run -- main.persi
```

بدون آرگومان، فایل `main.persi` خوانده می‌شود.

## build توسعه و release

```powershell
cargo build
cargo build --release
```

خروجی Windows:

```text
target/debug/persica.exe
target/release/persica.exe
```

نسخه‌ی release بهینه‌سازی، LTO و strip را فعال می‌کند.

## بررسی نصب

```powershell
.\target\release\persica.exe --version
.\target\release\persica.exe --eval "print(20 + 22);"
```

خروجی مورد انتظار:

```text
Persica 0.7.0
42
```

## بررسی کیفیت build

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

انتشار فقط زمانی معتبر است که هر سه دستور بدون خطا تمام شوند.
