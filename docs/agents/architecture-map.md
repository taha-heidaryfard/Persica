# نقشه‌ی سریع کد برای Agents

## Pipeline اصلی

```text
src/lib.rs::compile
  ├─ lexer::tokenize
  ├─ parser::parse
  └─ semantic::validate

src/lib.rs::run
  └─ runtime::Interpreter::interpret

src/main.rs
  ├─ parse_options
  ├─ compile
  └─ Interpreter::with_stdio + interpret_text
```

## مسئولیت فایل‌ها

| فایل / ماژول | مسئولیت | هنگام چه تغییری ویرایش شود |
|---|---|---|
| `src/source.rs` | `Span` و merge | مدل موقعیت منبع |
| `src/diagnostic.rs` | phase و rendering | قالب یا نوع خطا |
| `src/ast.rs` | مدل type، expression و statement | node یا type جدید |
| `src/lexer/mod.rs` | اسکنر، فاصله‌ها، توضیحات و اعداد | رفتار پیمایش یا اسکن |
| `src/lexer/token.rs` | توکن‌ها و نوع‌های واژگانی | syntax lexical یا کلمه‌ی کلیدی جدید |
| `src/parser/mod.rs` | هدایت جریان توکن‌ها و خطایابی | پیمایش و مصرف توکن‌ها |
| `src/parser/expr.rs` | تقدم عملگرها و عبارات | operator یا نحو عبارتی جدید |
| `src/parser/stmt.rs` | دستورها، تعریف‌ها، توابع و بلوک‌ها | statement یا ساختار کنترلی جدید |
| `src/semantic/mod.rs` | تحلیل‌گر، ثبت توابع و دامنه‌ها | جریان اعتبارسنجی کلی |
| `src/semantic/symbol.rs` | جدول نمادها، امضای توابع و هلپرها | ساختار symbol یا بررسی نوع پایه |
| `src/semantic/builtins.rs` | اسامی رزرو و استنتاج نوع توابع داخلی | افزودن تابع داخلی جدید |
| `src/semantic/expr.rs` | تحلیل معنایی و بررسی نوع عبارات | قاعده‌ی نوع‌سنجی عبارت‌ها |
| `src/semantic/stmt.rs` | تحلیل معنایی دستورها و بازگشت توابع | قواعد کنترلی و دستوری جدید |
| `src/runtime/mod.rs` | تعریف مفسر، ورودی/خروجی و دامنه‌ها | مدیریت چرخه‌ی حیات runtime |
| `src/runtime/value.rs` | مدل داده‌ی `Value` و نمایش متنی | نوع داده‌ی runtime جدید |
| `src/runtime/eval.rs` | اجرای دستورات و ارزیابی عبارات | رفتار دستورات و عملگرها |
| `src/runtime/builtins.rs` | پیاده‌سازی و توزیع توابع داخلی | افزودن یا اصلاح رفتار built-inها |
| `src/lib.rs` | public pipeline | API سطح بالا |
| `src/main.rs` | CLI، filesystem و stdio | option یا رفتار command line |

## تست‌ها

| فایل | دامنه |
|---|---|
| `tests/language.rs` | compatibility هسته‌ی 0.1 |
| `tests/version_05.rs` | قابلیت‌های زبان 0.5 و edge caseها |
| `tests/version_06.rs` | `final`، for-in، input/print و range جدید |
| `tests/version_07.rs` | float و built-inهای متن، عدد و آرایه |
| `tests/version_07_1.rs` | توابع آماری (mean، median، variance، stdev) و min/max آرایه |
| `tests/cli.rs` | binary، stdin، stdout/stderr و exit code |
| unit test داخل lexer | جزئیات lexical |

## مسیر تغییر رایج

### افزودن statement

`TokenKind/keyword → StmtKind → parser::statement → semantic::statement → runtime::execute → tests → docs`

### افزودن expression/operator

`TokenKind → ExprKind/BinaryOp → parser precedence → semantic::expression_type → runtime::evaluate → tests → docs`

### افزودن built-in

`semantic reserved-name + call_type → runtime::call → tests → docs/learn/language/builtins.md`

`print` یک statement ویژه است و مسیر آن `StmtKind::Print → parser::print_statement → semantic::statement → runtime::execute` است؛ آن را مانند built-in عادی تغییر نده.

### افزودن type

`ast::Type + lexer keyword → parser::parse_type → semantic compatibility → runtime::Value/value_type → operators/built-ins → tests/docs`
