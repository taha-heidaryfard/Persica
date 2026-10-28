# معماری Persica

Persica یک interpreter چندمرحله‌ای است:

```text
UTF-8 source
    ↓ lexer::tokenize
Vec<Token>
    ↓ parser::parse
Program (AST)
    ↓ semantic::validate
Validated Program
    ↓ runtime::Interpreter
Output یا Runtime Diagnostic
```

## مرز مراحل

### Lexer

تنها مسئول تبدیل نویسه به token و تشخیص خطاهای lexical است. درباره‌ی نوع متغیر یا ساختار expression تصمیم نمی‌گیرد.

### Parser

grammar و تقدم عملگرها را اعمال و AST می‌سازد. parser نوع expression را بررسی نمی‌کند.

### Semantic analyzer

scope، symbol، signature تابع، نوع expression و اعتبار control flow را بررسی می‌کند. AST را اجرا یا mutate نمی‌کند.

### Runtime

AST تأییدشده را اجرا می‌کند. با این حال برای ایمنی API، runtime در برابر AST نامعتبر نیز diagnostic تولید می‌کند و به semantic pass کورکورانه اعتماد نمی‌کند.

## API سطح بالا

`compile(source)` سه مرحله‌ی lexer، parser و semantic را اجرا و `Compilation { tokens, program }` برمی‌گرداند. `run(source)` پس از compile یک interpreter بدون ورودی host اجرا می‌کند.

CLI مستقیماً از `compile` استفاده می‌کند تا بتواند token و AST را قبل از اجرا نمایش دهد.

## اصول طراحی

- بدون dependency خارجی
- AST دارای ownership مستقل از source
- spanها با byte offset و line/column انسانی
- خطا به‌صورت value، بدون panic برای خطای برنامه‌ی کاربر
- بررسی نوع پیش از اجرا
- خروجی منطقی `Vec<String>` و خروجی متنی دقیق برای testability و embedding
- ورودی قابل تزریق برای تست

## محدودیت معماری فعلی

Interpreter tree-walk است و AST را مستقیم اجرا می‌کند. برای workload بزرگ، مرحله‌ی آینده تبدیل AST به bytecode و اجرای VM است. module loader، persistent session و stack trace چندفریمی هنوز وجود ندارند.

## آینده: کتابخانه‌ی استاندارد در خود Persica (Prelude)

برای ساده‌سازی توسعه‌ی کتابخانه استاندارد و اجتناب از تعریف دستی توابع سطح بالا در کد Rust، در نسخه‌های آینده توابع کمکی با خود زبان Persica در فایلی نظیر `src/stdlib/prelude.persi` نوشته شده و به صورت توکار (`include_str!`) توسط مفسر در زمان آغاز به عنوان Prelude اجرا و ثبت خواهند شد.
