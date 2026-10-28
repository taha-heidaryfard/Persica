# Persica Agent Instructions

این فایل برای تمام agentهایی است که در این repository کار می‌کنند.

## ابتدا بخوان

- مخزن دانش و حافظه پایدار آبسیدین: `Agent Memory/Persica/00 - Index.md`
- راهنمای اصلی agent: `docs/agents/README.md`
- نقشه‌ی فایل‌ها: `docs/agents/architecture-map.md`
- invariantها: `docs/agents/coding-rules.md`
- ماتریس تست: `docs/agents/verification-matrix.md`

## قواعد اجباری

1. خطای برنامه‌ی کاربر نباید Rust panic ایجاد کند.
2. تغییر language feature باید lexer/parser/AST/semantic/runtime را در صورت ارتباط هماهنگ کند.
3. هر bug fix regression test و هر feature تست مثبت و منفی می‌خواهد.
4. `cargo fmt --all -- --check`، `cargo clippy --all-targets -- -D warnings` و `cargo test --all-targets` پیش از تحویل اجرا شوند.
5. مستندات user در `docs/learn`، مستندات implementation در `docs/project` و دستورهای agent در `docs/agents` قرار می‌گیرند.
6. `target/` generated است و نباید ویرایش شود.
7. تغییرات نامرتبط موجود در workspace حفظ شوند.

## مسیرهای اصلی

```text
src/                 implementation
tests/               automated behavior
examples/            executable language examples
docs/learn/           learning Persica
docs/agents/          operational AI context
docs/project/         compiler and project internals
```
