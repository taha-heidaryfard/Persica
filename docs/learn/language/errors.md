# مدل خطاها

هر خطا شامل phase، پیام و در صورت امکان span منبع است.

## Lexer error

برای نویسه‌ی ناشناخته، escape نامعتبر، رشته‌ی بسته‌نشده یا block comment ناقص.

## Parser error

برای token نامنتظر، semicolon یا delimiter گم‌شده، assignment target نامعتبر و literal عددی خارج از بازه.

## Semantic error

پیش از اجرا برای مواردی مانند:

- متغیر یا تابع تعریف‌نشده
- mismatch نوع
- declaration تکراری
- call با arity اشتباه
- `break` بیرون حلقه
- `return` بیرون تابع
- مسیر return ناقص
- index غیر `int`

## Runtime error

خطاهایی که به مقدار واقعی وابسته‌اند:

- تقسیم یا remainder بر صفر
- overflow arithmetic
- index خارج محدوده
- تبدیل نامعتبر `to_int`
- پایان ورودی
- عبور از محدودیت recursion یا `range`

## قالب diagnostic

```text
semantic error: expected int, found bool
 --> app.persi:1:11
  |
 1 | int age = true;
  |           ^^^^
```

CLI diagnostic را روی stderr می‌نویسد و با exit code غیرصفر تمام می‌شود. `--check` فقط خطاهای lexer، parser و semantic را پیدا می‌کند؛ خطای runtime نیازمند اجراست.
