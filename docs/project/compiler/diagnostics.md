# Diagnostic System

پیاده‌سازی: `src/diagnostic.rs` و `src/source.rs`

## Span

```text
start, end: byte offsets
line, column: 1-based start position
```

Byte offset برای slicing امن UTF-8 و line/column برای نمایش انسانی استفاده می‌شود. `Span::merge` محدوده‌ی node مرکب را می‌سازد.

## Diagnostic

هر diagnostic شامل:

- `Phase`: Io، Lexer، Parser، Semantic یا Runtime
- `message`
- `Option<Span>`

خطای command line یا filesystem ممکن است span نداشته باشد.

## Rendering

`render(file_name, source)` خط مرتبط و marker را تولید می‌کند. عرض marker براساس تعداد نویسه‌هاست، نه byteها.

## قرارداد پیام

- پیام با حرف کوچک و بدون نقطه‌ی اجباری نوشته می‌شود.
- نام symbol داخل single quote می‌آید.
- expected و actual هر دو ذکر می‌شوند.
- runtime error باید operation یا index واقعی را مشخص کند.

## Fail-fast فعلی

Pipeline اولین خطا را برمی‌گرداند. API آینده برای IDE باید `Vec<Diagnostic>` و recovery parser ارائه کند؛ تغییر آن نیازمند release note و migration guide است.

