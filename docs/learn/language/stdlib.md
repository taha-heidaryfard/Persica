# کتابخانه استاندارد خود‌میزبان و پیش‌درآمد (Persica Prelude & Stdlib)

در نسخه **0.11.0** زبان پرسیان، **پیش‌درآمد خود‌میزبان (Self-hosted Prelude)** و **ماژول‌های استاندارد درونی (`std.*`)** پیاده‌سازی شدند. تمام این ساختارهای داده و توابع با ۱۰۰٪ کد خالص پرسیان (Dogfooding) نوشته شده‌اند و به صورت توکار (Zero Dependency) درون باینری کامپایلر قرار دارند.

---

## ۱. پیش‌درآمد عمومی (Core Auto-Prelude)

این اجزا در **تمامی کدهای پرسیان به صورت خودکار و بدون نیاز به نوشتن `import`** در دسترس هستند:

### الف) ساختارهای داده
- **`class Stack`:** پشته LIFO
  ```persica
  Stack s = Stack(); // سازنده پیش‌فرض ۰ آرگومانی
  s.push("task1");
  s.push("task2");
  print(s.pop());   // task2
  print(s.peek());  // task1
  print(s.size());  // 1
  ```
- **`class Queue`:** صف FIFO
  ```persica
  Queue q = Queue();
  q.enqueue(10);
  q.enqueue(20);
  print(q.dequeue()); // 10
  ```
- **`class Set`:** مجموعه یکتا بر پایه دیکشنری
  ```persica
  Set set = Set();
  set.add("apple");
  set.add("apple"); // مقدار تکراری اضافه نمی‌شود
  print(set.contains("apple")); // true
  set.remove("apple");
  ```
- **`class StringBuilder`:** الحاق و ساخت بهینه رشته‌ها
  ```persica
  StringBuilder sb = StringBuilder();
  sb.append_str("Hello ");
  sb.append_str("Persica");
  print(sb.to_str()); // Hello Persica
  ```

### ب) مدل‌های پایه (Structs & Results)
- **`struct Pair { any first; any second; }`**
- **`struct Result { bool ok; any value; string error; }`** به همراه توابع `ok(val)` و `err(msg)`:
  ```persica
  Result r = ok(100);
  if r.ok {
      print("Value:", r.value);
  }
  ```
- **`struct Option { bool has_value; any value; }`** به همراه توابع `some(val)` و `none()`.

### ج) الگوریتم‌های در دسترس
- **`zip(a, b)`:** ترکیب متناظر دو آرایه
- **`chunks(arr, size)`:** دسته‌بندی آرایه به قطعات دلخواه
- **`flatten(matrix)`:** تبدیل آرایه دوبعدی به یک‌بعدی
- **`binary_search(sorted_arr, target)`:** جستجوی دودویی سریع

---

## ۲. ماژول‌های استاندارد و ساختار سلسله‌مراتبی پوشه‌ها (`std.*`, `strings.*`)

کتابخانه استاندارد پرسیان ساختار درختی و سلسله‌مراتبی فیزیکی دارد:

```text
src/stdlib/
├── prelude.persi          # تعاریف خودکار لود شونده
├── std/                   # پوشه بسته std
│   ├── mod.persi
│   ├── collections.persi
│   ├── algorithms.persi
│   ├── strings.persi
│   └── math.persi
├── strings/               # پوشه ماژول strings و زیرپوشه‌ها
│   ├── mod.persi
│   └── ali/
│       └── mod.persi      # زیرپوشه و زیربسته strings.ali
├── collections/
│   └── mod.persi
├── algorithms/
│   └── mod.persi
└── math/
    └── mod.persi
```

### الف) ایمپورت ماژول‌های استاندارد
می‌توانید مستقیماً از پوشه `std` یا از نام اختصاصی پکیج‌ها استفاده کنید:

```persica
// ایمپورت از طریق پکیج std
from std.strings import repeat_str, center, truncate;
from std.collections import Stack, Queue, Set, StringBuilder;
from std.algorithms import binary_search, fill, zip, chunks;
from std.math import lerp, distance_2d;

// یا مستقیماً از پوشه اختصاصی ماژول
from strings import repeat_str, center, truncate;
from collections import Stack, Queue;
```

### ب) پکیج‌ها و زیرپوشه‌های تو در تو (Nested Subpackages)
اگر پکیجی دارای پوشه یا زیرپوشه باشد (مانند `strings.ali` که در پوشه `strings/ali/` قرار دارد)، ایمپورت سلسله‌مراتبی با دات ساپورت می‌شود:

```persica
from strings.ali import greet_ali, shout;

print(greet_ali("Persica")); // Salam Persica az strings.ali!
print(shout("Speed"));       // Speed!!!
```

### ج) ایمپورت فایل‌ها و پوشه‌های دلخواه روی دیسک
سامانه ماژولار پرسیان مسیر دات را دقیقاً به ساختار فولدری روی دیسک نگاشت می‌کند:
- `from my_pkg.sub.tool import my_func;` به ترتیب زیر را در پوشه فایل جاری و پوشه ریشه پروژه بررسی می‌کند:
  1. `my_pkg/sub/tool.persi`
  2. `my_pkg/sub/tool/mod.persi`
  3. `my_pkg/sub/tool/index.persi`
- فایل‌های فراخوانی‌شده تو در تو، ایمپورت‌های بعدی را بر اساس دایرکتوری نسبی خودشان حل‌وفصل (Resolve) می‌کنند.

---

## ۳. قابلیت سایه‌اندازی (Shadowing)
اگر کاربر تابعی با نام مشابه توابع کتابخانه استاندارد تعریف کند (مثلاً `def int binary_search(...)`)، تعریف کاربر با اولویت بالاتر فراخوانی شده و تعاریف Prelude را سایه‌اندازی (Shadow) می‌کند.
