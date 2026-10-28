# قواعد کدنویسی و Invariantها برای Agents

## Invariantهای pipeline

- token stream موفق همیشه با `Eof` پایان می‌یابد.
- parser فقط روی token stream دارای `Eof` مسیر عادی اجرا می‌شود.
- `compile` باید tokenize، parse و validate را به همین ترتیب اجرا کند.
- runtime باید diagnostic برگرداند، نه اینکه برای ورودی کاربر panic کند.
- تمام AST nodeهای قابل گزارش span دارند.

## UTF-8 و Span

- `Span.start/end` byte offset هستند.
- `line/column` یک‌مبنا و column براساس `char` است.
- source slicing فقط با byte offset معتبر انجام شود.
- طول marker diagnostic براساس نویسه محاسبه شود.

## Type system

- تبدیل ضمنی type اضافه نکن مگر با تصمیم مستند.
- `void` فقط return type تابع است.
- array همگن است و نوع empty literal باید از context بیاید.
- semantic و runtime باید هر دو assignment نامعتبر را امن مدیریت کنند.

## Scope و Control flow

- lookup از داخلی‌ترین scope شروع می‌شود.
- خروج از block باید scope را حتی در error path پاک کند.
- `continue` در `for` نباید increment را رد کند.
- `return` باید از block و loop تا call frame propagate شود.
- function declaration فقط top-level است.

## Functions و Built-ins

- signatureهای user function پیش از bodyها جمع می‌شوند.
- built-in nameها reserved هستند.
- arity/type در semantic و value-dependent failure در runtime بررسی می‌شود.
- recursion limit باید diagnostic بدهد.

## خطا

- phase صحیح انتخاب شود.
- پیام expected/actual را مشخص کند.
- نام symbol داخل `'...'` قرار گیرد.
- `unwrap` روی داده‌ی کنترل‌شده توسط source کاربر ممنوع است.

## Style و کیفیت

- formatter رسمی Rust مرجع format است.
- clippy warning به‌عنوان error در نظر گرفته می‌شود.
- enum match باید exhaustive بماند.
- helper فقط وقتی ساخته شود که invariant یا تکرار واقعی را متمرکز کند.

