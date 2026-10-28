# 03 - Language Syntax & Type System 📝

راهنمای جامع نحو (Syntax) و سیستم نوع داده زبان پرسیکا.

---

## ۱. انواع داده پایه (Primitive Types)

| نوع | توضیح | مثال |
| :--- | :--- | :--- |
| `int` | عدد صحیح علامت‌دار ۶۴ بیتی (`i64`) | `int count = 42;` |
| `float` | عدد اعشاری ۶۴ بیتی (`f64`) | `float price = 12.5;` |
| `bool` | بولی منطقی (`true` یا `false`) | `bool active = true;` |
| `string` | رشته متنی UTF-8 با شاخص‌های یونیکد | `string name = "طاها";` |
| `void` | فقط به عنوان نوع بازگشتی توابع فاقد مقدار | `def void greet() { ... }` |
| `any` | نوع آزاد و داینامیک برای هر مقدار | `any x = 10; x = "hello";` |

> [!NOTE] عدم تبدیل ضمنی (No Implicit Coercion)
> در پرسیکا هیچ‌گونه Type Coercion خودکار وجود ندارد. به عنوان مثال `1 + 2.0` خطا می‌دهد و باید صریحاً تبدیل شود: `to_float(1) + 2.0`.

---

## ۲. ساختارهای مجموعه‌ای (Collections)

### آرایه‌ها (`T[]`)
آرایه‌ها همگن هستند و می‌توانند چندبعدی یا تو در تو باشند:
```persica
int[] numbers = [10, 20, 30];
numbers[0] = 99; // جهش عنصر
print(numbers[0]); // 99
print(len(numbers)); // 3

string[][] matrix = [["a", "b"], ["c"]];
```

### دیکشنری و مپ (`map<Key, Value>` و `dict`)
- **حالت نوع‌دهی قوی و صریح (`map<K, V>`):**
  ```persica
  map<string, int> scores = {"ali": 20, "reza": 18};
  scores["sara"] = 19;
  ```
- **حالت آزاد به سبک پایتون (`dict` معادل `map<any, any>`):**
  ```persica
  dict user = {
      "name": "طاها",
      "age": 22,
      1: "رتبه اول"
  };
  print(user["name"]);
  print(user[1]);
  ```
- **پیمایش با حلقه:**
  ```persica
  for k in user {
      print(k, "->", user[k]);
  }
  ```

---

## ۳. متغیرها و ثوابت (`final`)

```persica
int counter = 0;
counter = counter + 1; // مجاز

final string app_name = "Persica";
// app_name = "Other"; // خطای کامپایل: متغیر final قابل انتساب مجدد نیست

// اما در آرایه و مپ final، عناصر درون آن قابل ویرایش هستند:
final dict config = {"port": 8080};
config["port"] = 9000; // کاملاً مجاز
```

---

## ۴. کنترل جریان (Control Flow)

### شرط‌ها (`if`, `else`)
پرانتز برای شرط اختیاری است:
```persica
if (score >= 10) {
    print("قبول");
} else {
    print("مردود");
}
```

### حلقه‌ها (`while`, `for`, `for-in`)
```persica
// حلقه While
int i = 0;
while (i < 5) {
    i++;
}

// حلقه For کلاسیک
for (int j = 0; j < 10; j++) {
    if (j == 5) continue;
    print(j);
}

// حلقه For-in روی آرایه، رشته و دیکشنری (با یا بدون پرانتز)
for item in [1, 2, 3] { print(item); }
for (ch in "سلام") { print(ch); }
for key in user { print(key); }
```

---

## ۵. توابع (`def`)

توابع در سطح بالا (Top-level) تعریف می‌شوند و از نوع بازگشتی صریح و بازگشت زودهنگام پشتیبانی می‌کنند:
```persica
def int add(int a, int b) {
    return a + b;
}

def void log_status(bool ok) {
    if (!ok) return;
    print("سیستم فعال است");
}
```
