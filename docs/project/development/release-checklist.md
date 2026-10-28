# چک‌لیست انتشار

## Versioning

- [ ] نسخه‌ی `Cargo.toml` به‌روزرسانی شده است.
- [ ] `Cargo.lock` نسخه‌ی package را منعکس می‌کند.
- [ ] خروجی `persica --version` درست است.

## Code quality

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets -- -D warnings`
- [ ] `cargo test --all-targets`
- [ ] `cargo build --release`
- [ ] تمام مثال‌ها با `--check` معتبرند.

## Documentation

- [ ] release note مستقل ایجاد شده است.
- [ ] CHANGELOG خلاصه‌ی نسخه را دارد.
- [ ] README قابلیت‌های اصلی را نشان می‌دهد.
- [ ] topic doc تمام قابلیت‌های جدید را توضیح می‌دهد.
- [ ] migration guide برای breaking change وجود دارد.
- [ ] docs index به فایل‌های جدید لینک دارد.
- [ ] محدودیت‌ها صریح نوشته شده‌اند.

## Runtime verification

- [ ] `main.persi` با release binary اجرا می‌شود.
- [ ] stdout و exit code موفق‌اند.
- [ ] یک lexical، parser، semantic و runtime error دستی بررسی شده است.
- [ ] stdin و `read_line` در صورت تغییر I/O تست شده‌اند.

## Artifact

- [ ] binary release ساخته شده است.
- [ ] اندازه و مسیر artifact ثبت شده است.
- [ ] license همراه source است.
- [ ] هیچ فایل موقت یا secret وارد package نشده است.

