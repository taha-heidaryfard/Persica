# Playbook تغییرات برای Agents

## قابلیت جدید زبان

1. syntax نهایی و مثال را در `docs/learn/` تعریف کن.
2. lexical نیاز را بررسی کن؛ هر syntax جدید token تازه نمی‌خواهد.
3. AST را با کمترین variant روشن گسترش بده.
4. parser و span کامل را اضافه کن.
5. type/scope/control-flow rule را در semantic پیاده کن.
6. runtime behavior و خطاهای value-dependent را اضافه کن.
7. حداقل happy path، type error و runtime edge case تست کن.
8. release note و changelog را به‌روز کن.

## رفع bug

1. ابتدا تستی بنویس که روی رفتار فعلی fail شود.
2. علت را به phase مالک آن محدود کن.
3. fix کوچک و بدون تغییر نامرتبط اعمال کن.
4. regression suite کامل را اجرا کن.
5. bug fix را در release note نسخه‌ی بعد ثبت کن.

## تغییر grammar یا precedence

1. جدول precedence و grammar را قبل و بعد مقایسه کن.
2. ambiguity و associativity را صریح تعیین کن.
3. parser test برای ترکیب چند operator بنویس.
4. migration impact را ثبت کن؛ این تغییر بالقوه breaking است.

## Built-in جدید

1. نام را در reserved-name validation اضافه کن.
2. arity و type نتیجه را در semantic تعریف کن.
3. رفتار و error runtime را پیاده کن.
4. input/output host dependency را قابل تزریق نگه دار.
5. جدول built-in و embedding implications را مستند کن.

> **نکته معماری آینده (Persica-hosted Prelude):** طبق تصمیم ثبت‌شده در `ROADMAP.md`، در فازهای بعدی توابع سطح بالاتر کتابخانه استاندارد می‌توانند مستقیماً با خود سینتکس Persica در فایلی نظیر `src/stdlib/prelude.persi` نوشته شوند و مفسر به صورت خودکار آن را در آغاز لود کند تا نیازی به افزودن دستی کدهای Rust نباشد.

## تغییر CLI

1. option conflict و missing argument را مدیریت کن.
2. stdout برای خروجی عادی و stderr برای diagnostic بماند.
3. exit code موفق/ناموفق تست شود.
4. `tests/cli.rs` را با اجرای binary واقعی گسترش بده.
5. help text، CLI doc و README به‌روز شوند.

## Refactor داخلی

1. رفتار observable را با تست موجود freeze کن.
2. public API impact را بررسی کن.
3. refactor را از feature change جدا نگه دار.
4. fmt، clippy و کل test suite را اجرا کن.

