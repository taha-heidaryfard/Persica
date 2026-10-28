# Persica 0.7

Persica یک زبان برنامه‌نویسی تایپ‌شده و کوچک است که کاملاً با Rust پیاده‌سازی شده است. مسیر اجرای آن شامل lexer، recursive-descent parser، AST، semantic analyzer و tree-walk interpreter است و هیچ dependency خارجی ندارد.

## قابلیت‌ها

- انواع `int`، `float`، `bool`، `string`، `void` و آرایه‌های چندبعدی
- متغیرهای تایپ‌شده، ثابت‌های `final`، shadowing و scopeهای تودرتو
- `if/else`، `while`، حلقه‌ی ساده‌ی `for ... in` و حلقه‌ی C-style
- `break`، `continue` و `return`
- توابع تایپ‌شده، forward call و recursion
- array literal، indexing و mutation
- عملگرهای arithmetic، comparison و logical
- `%`، `+=`، `-=`, `*=`, `/=`, `%=`، `++` و `--`
- short-circuit واقعی برای `&&` و `||`
- شناسه‌های Unicode و فارسی
- line comment و block comment تودرتو
- ورودی و خروجی شبیه Python با `input(prompt)` و `print(..., sep=..., end=...)`
- built-inهای `len`، `range`، `append`، `to_string` و `to_int`
- ابزارهای متن مانند `trim`، `split`، `join`، `replace`، `find` و `slice`
- ابزارهای عددی مانند `abs`، `min`، `max`، `clamp`، `pow`، `sqrt` و rounding
- ابزارهای آرایه مانند `sort`، `sum`، `reverse`، `insert` و `remove`
- خطاهای source-aware در مراحل lexer، parser، semantic و runtime
- حالت‌های CLI برای check، eval، token dump و AST dump

فهرست مستندات در [docs/README.md](docs/README.md) به سه بخش تقسیم شده است: [یادگیری زبان](docs/learn/README.md)، [راهنمای Agents](docs/agents/README.md) و [کد و معماری پروژه](docs/project/README.md). مرجع فشرده‌ی زبان در [LANGUAGE.md](docs/learn/LANGUAGE.md) قرار دارد.

## اجرا

```powershell
cargo run -- main.persi
cargo run -- examples/fibonacci.persi
```

اگر فایل مشخص نشود، `main.persi` اجرا می‌شود. برای اجرای مستقیم source:

```powershell
cargo run -- --eval "print(20 + 22);"
```

بررسی بدون اجرا:

```powershell
cargo run -- --check main.persi
```

خواندن برنامه از stdin:

```powershell
Get-Content main.persi | cargo run -- -
```

ابزارهای بررسی compiler:

```powershell
cargo run -- --tokens main.persi
cargo run -- --ast main.persi
```

## تست و build

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo build --release
```

فایل اجرایی نهایی در `target/release/persica.exe` ساخته می‌شود.

## نمونه

```persica
int count = to_int(input("Count: "));
final string title = "Numbers";

print(title, end=": ");
for number in range(1, count + 1) {
    print(number, end=" ");
}
print();
```

خروجی:

```text
Count: 4
Numbers: 1 2 3 4
```

## ساختار

```text
src/
  lexer.rs       tokenization, Unicode and comments
  parser.rs      grammar and operator precedence
  ast.rs         typed syntax tree
  semantic.rs    scopes, names, types and control-flow checks
  runtime.rs     interpreter, functions, arrays and built-ins
  diagnostic.rs source-aware errors
  source.rs      source spans
  lib.rs         reusable compiler pipeline
  main.rs        command-line interface
tests/
  language.rs    v0.1 compatibility suite
  version_05.rs  v0.5 behavior and error suite
  version_06.rs  v0.6 syntax and I/O suite
  version_07.rs  v0.7 float, text, number and array suite
```

## مجوز

MIT
