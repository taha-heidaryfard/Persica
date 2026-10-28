# استفاده از Persica به‌عنوان کتابخانه‌ی Rust

crate هم binary و هم library تولید می‌کند.

## Compile بدون اجرا

```rust
let compilation = persica::compile("int answer = 42;")?;
println!("{} tokens", compilation.tokens.len());
println!("{:#?}", compilation.program);
```

`compile` lexer، parser و semantic analyzer را اجرا می‌کند.

## اجرای ساده

```rust
let output = persica::run("print(6 * 7);")?;
assert_eq!(output, ["42"]);
```

`run` ورودی host را غیرفعال می‌کند. برنامه‌ی دارای `input()` یا `read_line()` باید Interpreter سفارشی داشته باشد.

## Interpreter با ورودی تستی

```rust
let compilation = persica::compile(
    "string name = input(); print(\"Hello\", name);"
)?;

let output = persica::runtime::Interpreter::with_input(["Taha\n"])
    .interpret(&compilation.program)?;
```

## Interpreter متصل به stdin

```rust
let output = persica::runtime::Interpreter::with_stdin()
    .interpret(&compilation.program)?;
```

## متن دقیق و stdio تعاملی

`interpret` برای سازگاری، محتوای هر فراخوانی print را به‌صورت `Vec<String>` می‌دهد. برای حفظ دقیق `sep`، `end` و prompt از `interpret_text` استفاده کنید:

```rust
let text = persica::runtime::Interpreter::with_input(["Ali\n"])
    .interpret_text(&compilation.program)?;
```

CLI از `Interpreter::with_stdio()` استفاده می‌کند تا prompt پیش از انتظار برای input فوراً نمایش داده و flush شود.

## نمایش Diagnostic

```rust
match persica::compile(source) {
    Ok(compilation) => { /* use it */ }
    Err(error) => eprintln!("{}", error.render("app.persi", source)),
}
```

## APIهای سطح پایین

ماژول‌های `lexer`، `parser`، `semantic`، `ast` و `source` public هستند و می‌توان مراحل را جدا اجرا کرد. ترتیب معتبر همیشه tokenize، parse و validate است. اجرای AST بدون validate برای host توصیه نمی‌شود.

## سازگاری

API کتابخانه هنوز pre-1.0 است. تغییر enumهای AST یا signatureها می‌تواند breaking باشد و باید در سند release مربوط ثبت شود.
