# 08 - Future Roadmap 🚀

نقشه راه و اهداف توسعه زبان Persica در فازهای آینده.

---

## 🎯 فازهای آتی به ترتیب اولویت

### ۱. ساختارهای داده تعریف‌شده توسط کاربر (`struct`)
تعریف مدل‌های داده‌ای با فیلدهای دارای نام و نوع مشخص:
```persica
struct User {
    string name,
    int age,
    bool is_active
}

User u = User("طاها", 22, true);
print(u.name);
```

### ۲. سیستم ماژولار و وارد کردن فایل‌ها (`import`)
امکان تقسیم پروژه‌های بزرگ به چند فایل و اشتراک‌گذاری توابع و ساختارها:
```persica
import "utils.persi";
// یا
import { calculate_tax } from "finance.persi";
```

### ۳. خودمیزبانی توابع استاندارد (Persica Prelude)
نوشتن توابع سطح بالای استاندارد با خود زبان Persica در فایلی نظیر `src/stdlib/prelude.persi` و بارگذاری اولیه آن با `include_str!` توسط مفسر.

### ۴. موتور بایت‌کد و ماشین مجازی (Bytecode VM)
جایگزینی مفسر درختی (Tree-walk Interpreter) با یک ماشین مجازی مبتنی بر پشته (Stack-based Virtual Machine) جهت افزایش ۱۰ تا ۵۰ برابری سرعت اجرای کدهای پرسیکا.

### ۵. پکیج منیجر و ابزار CLI توسعه
ساخت ابزاری شبیه `cargo` یا `npm` برای مدیریت پکیج‌ها و ایجاد پروژه‌های جدید در پرسیکا (`persica new`, `persica run`, `persica test`).
