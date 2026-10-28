# رابط خط فرمان Persica

## Synopsis

```text
persica [OPTIONS] [FILE]
persica [OPTIONS] --eval SOURCE
```

اگر `FILE` حذف شود، `main.persi` استفاده می‌شود.

## گزینه‌ها

| گزینه | کاربرد |
|---|---|
| `--tokens` | نمایش tokenهای تولیدشده |
| `--ast` | نمایش AST با قالب debug |
| `--check` | lexer، parser و semantic check بدون اجرا |
| `-e`, `--eval` | اجرای source دریافت‌شده از command line |
| `-V`, `--version` | نمایش نسخه |
| `-h`, `--help` | نمایش راهنما |

## اجرای فایل

```powershell
persica app.persi
```

کد خروج موفق `0` و خطا `1` است. خروجی برنامه روی stdout و diagnostics روی stderr نوشته می‌شوند.

## Check mode

```powershell
persica --check app.persi
```

این حالت runtime را اجرا نمی‌کند. بنابراین برنامه‌ای مانند `print(1 / 0);` check می‌شود، اما هنگام اجرای واقعی خطای تقسیم بر صفر می‌دهد.

## Eval mode

```powershell
persica --eval "def int twice(int n) { return n * 2; } print(twice(21));"
```

فقط یک منبع ورودی مجاز است؛ نمی‌توان `--eval` و مسیر فایل را هم‌زمان داد.

## stdin به‌عنوان source

نام فایل `-` به معنی خواندن کل source از stdin است:

```powershell
Get-Content app.persi | persica -
```

در این حالت stdin برای source مصرف می‌شود؛ بنابراین برنامه نباید بعداً از `input()` یا `read_line()` انتظار ورودی بیشتری داشته باشد، مگر host ورودی جداگانه‌ای فراهم کند.

## Token dump

```powershell
persica --tokens app.persi
```

هر سطر نوع token، lexeme و موقعیت `line:column` را نشان می‌دهد. token رشته، مقدار decodeشده را نمایش می‌دهد.

## AST dump

```powershell
persica --ast app.persi
```

این خروجی API پایدار محسوب نمی‌شود و برای debugging compiler است.

## ترکیب گزینه‌ها

```powershell
persica --check --tokens --ast app.persi
```

در این حالت token و AST نمایش داده می‌شود، semantic analysis انجام می‌شود و برنامه اجرا نمی‌شود.
