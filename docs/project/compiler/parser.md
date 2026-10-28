# Parser و Grammar

پیاده‌سازی: `src/parser.rs`

Parser از نوع hand-written recursive descent است و روی slice از tokenها حرکت می‌کند.

## Entry point

```rust
pub fn parse(tokens: &[Token]) -> Result<Program, Diagnostic>
```

ورودی خالی بدون `Eof` نیز به‌صورت diagnostic رد می‌شود.

## Declaration dispatch

`declaration` ابتدا `def`، سپس `final` یا type start و بعد statement معمولی را بررسی می‌کند. این ترتیب تفاوت declaration با expression را مشخص می‌کند.

## Expression precedence

هر سطح grammar تابع جداگانه دارد:

```text
assignment
logical_or
logical_and
equality
comparison
term
factor
unary
postfix
primary
```

سطوح binary از helper به نام `binary_level` استفاده می‌کنند. assignment recursive و راست‌گرا است.

## Desugaring محدود

Compound assignment و `++/--` هنگام parse به ترکیب assignment و binary expression تبدیل می‌شوند. `for` به `while` تبدیل نمی‌شود، چون `continue` باید increment را درست اجرا کند؛ در نتیجه variant مستقل AST دارد.

دو گونه‌ی حلقه variant مستقل دارند: `For` برای شکل C-style و `ForEach` برای `for name in expression`. شرط‌های `if` و `while` پرانتز اختیاری دارند. parser چاپ، آرگومان‌های positional و دو گزینه‌ی نام‌دار `sep` و `end` را جدا می‌کند و گزینه‌ی تکراری یا ناشناخته را رد می‌کند.

## Call و indexing

مرحله‌ی postfix چند call یا index متوالی را می‌پذیرد. call فعلاً فقط callee نام‌دار دارد و first-class function وجود ندارد.

## Array type

`parse_type` پس از type پایه هر تعداد `[]` را می‌خواند و نوع تو‌در‌تو می‌سازد. `void` فقط در return type تابع پذیرفته می‌شود.

## Span construction

span node مرکب از merge کردن token شروع و آخر ساخته می‌شود. در binary expression span از ابتدای left تا انتهای right است.

## Error recovery

Parser فعلاً fail-fast است. برای LSP آینده باید synchronization روی `;`، `}` و keywordهای declaration اضافه شود تا چند diagnostic در یک pass تولید شود.

Grammar کامل زبان در [LANGUAGE.md](../../learn/LANGUAGE.md) ثبت شده است.
