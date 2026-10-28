# Workflow توسعه

## ساختار مخزن

```text
src/          implementation
tests/        integration and CLI tests
examples/     executable Persica examples
docs/         topic and release documentation
```

## ترتیب تغییر یک قابلیت زبان

1. syntax و semantics موردنظر را در issue یا سند طراحی مشخص کنید.
2. tokenهای لازم را به lexer اضافه کنید.
3. AST را گسترش دهید.
4. parser و spanها را پیاده‌سازی کنید.
5. semantic ruleها را اضافه کنید.
6. runtime behavior را پیاده‌سازی کنید.
7. positive، negative و regression test بنویسید.
8. سند user-facing در `docs/learn` و سند implementation در `docs/project` را به‌روزرسانی کنید.
9. release note نسخه را تکمیل کنید.
10. fmt، clippy، test و release build را اجرا کنید.

## اصل pipeline سبز

پس از هر لایه باید پروژه compile شود یا تغییر در یک commit منطقی و کامل نگه داشته شود. تغییر syntax بدون semantic/runtime و test کامل محسوب نمی‌شود.

## Style

- خطای برنامه‌ی Persica نباید باعث panic Rust شود.
- برای وضعیت غیرممکن داخلی `expect` با پیام مشخص قابل استفاده است.
- type و operator باید enum باشند، نه string dispatch پراکنده.
- I/O باید از core قابل جداشدن باشد تا تست deterministic باقی بماند.
- dependency جدید فقط با دلیل روشن و ثبت در release note اضافه شود.

## دستورات روزمره

```powershell
cargo fmt --all
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo run -- --check examples/features.persi
```

## Breaking change

تغییر grammar، precedence، نوع runtime، AST public یا رفتار built-in breaking محسوب می‌شود و migration guide می‌خواهد.
