# Lexer

پیاده‌سازی: `src/lexer.rs`

## ورودی و خروجی

```rust
pub fn tokenize(source: &str) -> Result<Vec<Token>, Diagnostic>
```

خروجی همیشه در حالت موفق با token `Eof` پایان می‌یابد.

## Token

هر token شامل این موارد است:

- `kind`: عضو `TokenKind`
- `lexeme`: متن token؛ برای string مقدار decodeشده
- `span`: byte range و line/column شروع

## پیمایش UTF-8

فیلد `current` byte offset است. `advance` یک `char` کامل مصرف و به‌اندازه‌ی `len_utf8` جلو می‌رود. `column` براساس نویسه افزایش می‌یابد، نه byte.

این تفاوت مهم است: slicing source با byte offset انجام می‌شود، ولی diagnostic ستون قابل فهم برای کاربر نمایش می‌دهد.

## ترتیب تشخیص

1. whitespace
2. delimiter و punctuation
3. operator تک یا دو نویسه‌ای
4. comment پس از `/`
5. string
6. number ASCII
7. identifier یا keyword Unicode
8. unexpected character

عدد با digit شروع می‌شود. اگر پس از بخش صحیح، `.` و حداقل یک digit وجود داشته باشد همان token عددی به‌عنوان literal اعشاری parse می‌شود. در غیر این صورت literal صحیح است. notation علمی هنوز پشتیبانی نمی‌شود.

## Comment

`//` تا newline و `/* */` با depth counter پردازش می‌شود؛ بنابراین block comment تودرتو است.

## String

Lexer escapeها را decode و مقدار نهایی را در `lexeme` قرار می‌دهد. newline مستقیم و EOF پیش از quote پایان خطا هستند.

## Keyword

پس از اسکن identifier، متن با جدول keywordهای داخل `identifier` مقایسه می‌شود. اضافه‌کردن keyword جدید نیازمند:

در 0.6، `final` و `in` keyword شدند و در 0.7، `float` به keywordهای نوع اضافه شد. `sep` و `end` keyword عمومی نیستند؛ parser فقط در context آرگومان‌های `print` آن‌ها را به‌عنوان نام گزینه تشخیص می‌دهد، بنابراین در بقیه‌ی زبان می‌توانند identifier باشند.

1. variant جدید در `TokenKind`
2. mapping متن در lexer
3. مصرف token در parser
4. تست lexical و language-level

## خطا و recovery

Lexer فعلاً fail-fast است و اولین diagnostic را برمی‌گرداند. جمع‌آوری چند خطای lexical در roadmap ابزارهای توسعه قرار دارد.
