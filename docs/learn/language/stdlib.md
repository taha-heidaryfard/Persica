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

## ۲. ماژول‌های استاندارد (`std.*`)

علاوه بر Prelude خودکار، می‌توانید از ماژول‌های تخصصی استاندارد با سینتکس ایمپورت پایتونی استفاده کنید:

```persica
from std.collections import Stack, Queue, Set, StringBuilder;
from std.algorithms import binary_search, fill, zip, chunks;
from std.strings import repeat_str, center, truncate;
from std.math import lerp, distance_2d, deg_to_rad, rad_to_deg;

print(center("MENU", 14, "=")); // =====MENU=====
print(lerp(0.0, 100.0, 0.5));   // 50.0
```

---

## ۳. قابلیت سایه‌اندازی (Shadowing)
اگر کاربر تابعی با نام مشابه توابع کتابخانه استاندارد تعریف کند (مثلاً `def int binary_search(...)`)، تعریف کاربر با اولویت بالاتر فراخوانی شده و تعاریف Prelude را سایه‌اندازی (Shadow) می‌کند.
