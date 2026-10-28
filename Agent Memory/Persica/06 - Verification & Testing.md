# 06 - Verification & Testing 🧪

دستورالعمل‌ها و پروتکل اعتبارسنجی کیفیت کد، تست‌ها و ساخت باینری نهایی.

---

## ⚡ چک‌لیست اعتبارسنجی (Execution Checklist)

همیشه قبل از پایان کار یا ثبت کامیت گیت، این دنباله از دستورات را اجرا کنید:

```powershell
# ۱. بررسی استایل و قالب‌بندی کد با rustfmt
& "C:\Users\Taxtander\.cargo\bin\cargo.exe" fmt --all -- --check

# ۲. اجرای لینتر راست با جلوگیری از هرگونه هشدار
& "C:\Users\Taxtander\.cargo\bin\cargo.exe" clippy --all-targets -- -D warnings

# ۳. اجرای تمام آزمون‌های خودکار
& "C:\Users\Taxtander\.cargo\bin\cargo.exe" test --all-targets

# ۴. کامپایل نسخه بهینه‌شده ریلیز
& "C:\Users\Taxtander\.cargo\bin\cargo.exe" build --release

# ۵. اعتبارسنجی تمام نمونه کدهای مثال
Get-ChildItem examples/*.persi | ForEach-Object { .\target\release\persica.exe --check $_.FullName }
```

---

## 📊 ماتریس تست‌های پروژه (۸۱ تست)

| سوئیت تست | تعداد تست | حوزه پوشش |
| :--- | :---: | :--- |
| `src/lib.rs` (Unit) | ۲ | اسکن رشته‌ها، کاراکترهای اسکیپ، شناسه‌ها و کامنت‌های یونیکد |
| `tests/cli.rs` | ۵ | تست فلگ‌های خط فرمان، `-e`, `--check`, stderr, stdin |
| `tests/language.rs` | ۱۱ | هسته پایه زبان v0.1: تقدم عملگرها، اتصال، تقسیم بر صفر، سرریز عدد صحیح |
| `tests/version_05.rs` | ۱۲ | توابع کاربر، توابع بازگشتی (`factorial`), آرایه‌ها، ایندکس و اسلایس |
| `tests/version_06.rs` | ۹ | متغیرهای `final`، حلقه‌های `for-in`، تابع `range`، آرگومان‌های نام‌دار `print` |
| `tests/version_07.rs` | ۱۰ | نوع داده `float`، تبدیل‌های صریح عددی، ابزارهای متنی پیشرفته |
| `tests/version_07_1.rs` | ۷ | توابع `min` و `max` روی آرایه‌ها و اسکالرها، توابع آماری پایه |
| `tests/builtins_registry.rs` | ۱۵ | تست سیستم رجیستری و بیش از ۱۱۵ تابع، خطاهای دامنه بدون panic، Shadowing |
| `tests/version_08.rs` | ۱۰ | تست جامع `map<K, V>` و `dict`، انتساب ایندکس، پیمایش `for-in`، خطاهای تایپ |
| **مجموع** | **۸۱** | **۱۰۰٪ موفقیت‌آمیز** |
