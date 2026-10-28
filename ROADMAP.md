# Persica Roadmap

رفتار نسخه‌های منتشرشده در [release docs](docs/project/releases/) و قانون مستندسازی آپدیت‌ها در [documentation-policy.md](docs/project/development/documentation-policy.md) ثبت شده است.

## Completed through 0.7.1

- Typed variables and expressions
- Control flow and lexical scopes
- Typed functions and recursion
- Arrays and core collection helpers
- Source-aware diagnostics and compiler inspection CLI
- Final bindings
- Python-style input, print, for-in and range syntax
- Exact-text and streaming output APIs
- Float numbers and explicit numeric conversion
- Rich Unicode text, numeric and array helpers
- Array extrema and statistics helpers (`min`, `max`, `mean`, `median`, `variance`, `stdev`)

## 0.8 — More in-memory data structures and Prelude

- Map/dictionary and set
- Tuple-like multi-value results
- Persica-hosted standard library / Prelude (تعریف توابع و ابزارهای سطح بالاتر مستقیماً با خود Persica در فایلی نظیر `src/stdlib/prelude.persi` و لود خودکار آن در زمان اجرای مفسر به جای پیاده‌سازی دستی در Rust)

## Deferred by current product direction

- `import` and multiple source files
- File I/O
- Structured error handling
- Persistent REPL
- Formatter, LSP and editor extensions

## 0.9 — Developer tooling

- Formatter
- Language Server Protocol implementation
- VS Code syntax highlighting and diagnostics
- Package manifest and dependency resolution

## 1.0 — Runtime evolution

- Bytecode compiler and virtual machine
- Stack traces with Persica function frames
- Debug information and benchmarks

## Future

- Structs and enums
- Pattern matching
- Generics
- WebAssembly or native-code backend
