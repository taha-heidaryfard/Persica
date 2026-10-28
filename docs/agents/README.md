# راهنمای Agents

این بخش context عملیاتی برای agentهای هوش مصنوعی و ابزارهای خودکار است. هدف آن کاهش حدس، جلوگیری از regression و مشخص‌کردن مسیر درست تغییرات است.

## شروع هر task

1. درخواست کاربر و محدوده‌ی تغییر را مشخص کن.
2. [نقشه‌ی کد](architecture-map.md) را برای فایل‌های مرتبط بخوان.
3. [Invariantها و قواعد کدنویسی](coding-rules.md) را رعایت کن.
4. recipe مناسب را از [راهنمای تغییرات](change-playbook.md) انتخاب کن.
5. تست لازم را از [ماتریس اعتبارسنجی](verification-matrix.md) اجرا کن.
6. طبق [قواعد مستندسازی agentها](documentation-rules.md) اسناد را هم‌زمان به‌روز کن.

## منابع حقیقت

| موضوع | مرجع |
|---|---|
| رفتار زبان | `docs/learn/language/` |
| grammar خلاصه | `docs/learn/LANGUAGE.md` |
| ساختار implementation | `docs/project/compiler/` |
| نسخه و package | `Cargo.toml` |
| رفتار اثبات‌شده | `tests/` |
| قابلیت‌های آینده | `ROADMAP.md` |
| تغییرات منتشرشده | `docs/project/releases/` |

اگر سند و تست اختلاف دارند، ابتدا اختلاف را گزارش و مشخص کن آیا bug در implementation است یا سند stale شده است؛ بدون بررسی یکی را کورکورانه تغییر نده.

## دستورات پایه

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo build --release
```

در محیط فعلی اگر `cargo` در PATH نبود:

```powershell
C:\Users\Taxtander\.cargo\bin\cargo.exe test --all-targets
```

## ممنوعیت‌های مهم

- `target/` را ویرایش یا commit نکن.
- برای خطای source کاربر panic ایجاد نکن.
- semantic check را دور نزن و AST خام را مسیر عادی اجرا نکن.
- capability جدید را بدون positive/negative test تحویل نده.
- grammar یا precedence را بدون آپدیت سند و release note تغییر نده.
- تغییرات نامرتبط کاربر را overwrite یا پاک نکن.

