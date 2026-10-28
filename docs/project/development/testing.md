# استراتژی تست

## لایه‌های فعلی

### Unit test

داخل `src/lexer.rs` برای رفتارهای isolated مانند Unicode identifier و string escape.

### Compatibility suite

`tests/language.rs` رفتار نسخه‌ی 0.1 را حفظ می‌کند: expression، assignment، typing، comment، خطا و short-circuit.

### Feature suite

`tests/version_05.rs` قابلیت‌های 0.5 را پوشش می‌دهد: scope، control flow، function، recursion، array، built-in و runtime error.

`tests/version_06.rs` سینتکس تایپ‌شده‌ی ساده، `final`، `for ... in`، شکل‌های `range`، `input` prompt و متن دقیق `print` را پوشش می‌دهد.

`tests/version_07.rs` نوع float، عملیات Unicode، توابع عدد و آرایه، type errorها و خطاهای وابسته به مقدار را پوشش می‌دهد.

### CLI suite

`tests/cli.rs` خود binary را اجرا می‌کند و stdout، stderr، exit code، eval، check و stdin را می‌سنجد.

## تست لازم برای قابلیت جدید

حداقل این موارد باید وجود داشته باشد:

1. happy path
2. precedence یا scope interaction
3. type error
4. syntax یا lexical error مرتبط
5. runtime edge case در صورت وجود
6. CLI یا embedding behavior در صورت تغییر interface
7. regression برای هر bug کشف‌شده

## اجرای تست‌ها

```powershell
cargo test --all-targets
cargo test --test version_05
cargo test --test version_06
cargo test --test version_07
cargo test test_name -- --exact
```

## تست مثال‌ها

تمام فایل‌های `examples/*.persi` باید حداقل با `--check` معتبر باشند. مثال تعاملی را می‌توان با stdin مشخص اجرا کرد.

## قواعد assertion

- phase خطا بررسی شود.
- بخش پایدار message بررسی شود، نه کل formatting مگر در تست diagnostic.
- output به‌ترتیب دقیق مقایسه شود.
- تست به ساعت، network یا وضعیت خارجی وابسته نباشد.

## هدف آینده

- snapshot test برای diagnostics
- property test lexer/parser
- fuzzing روی source تصادفی
- differential test بین tree-walk و VM آینده
- benchmark برای recursion، loop و array
