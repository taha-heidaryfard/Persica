# 04 - Builtin Functions Catalog 📚

پرسیکا دارای دو دسته تابع داخلی است:
1. **توابع هسته و وضعیت‌دار (Core State Builtins):** مستقیماً در Interpreter و Semantic مدیریت می‌شوند.
2. **توابع رجیستری توکار (Built-in Registry):** بیش از ۱۱۵ تابع خالص در `src/builtins/` که قابلیت Shadow شدن توسط توابع کاربر را دارند.

---

## ۱. توابع ویژه و هسته (Core Builtins)

| تابع | امضا | توضیح |
| :--- | :--- | :--- |
| `print` | `print(..., sep=" ", end="\n")` | چاپ مقادیر با جداکننده و انتهای سفارشی |
| `input` / `read_line` | `input(prompt="") -> string` | دریافت خط ورودی از کاربر / کنسول |
| `len` | `len(string \| array \| map) -> int` | طول رشته (کاراکتر یونیکد)، آرایه یا مپ |
| `range` | `range(stop)`, `range(start, stop, step=1) -> int[]` | تولید آرایه اعداد متوالی مثل پایتون |
| `to_string` | `to_string(value) -> string` | تبدیل هر مقدار به رشته متنی |
| `to_int` | `to_int(string \| float \| int) -> int` | تبدیل به عدد صحیح |
| `to_float` | `to_float(string \| int \| float) -> float` | تبدیل به عدد اعشاری |
| `contains_key` | `contains_key(map, key) -> bool` | بررسی وجود کلید در مپ/دیکشنری |
| `keys` | `keys(map) -> key_type[]` | استخراج تمام کلیدهای مپ به شکل آرایه |
| `values` | `values(map) -> value_type[]` | استخراج تمام مقادیر مپ به شکل آرایه |
| `remove_key` | `remove_key(map, key) -> map` | تولید مپ جدید فاقد کلید مشخص |
| `insert` | `insert(array, index, value) -> array` | درج عنصر در آرایه و بازگرداندن آرایه جدید |
| `remove` | `remove(array, index) -> array` | حذف عنصر از آرایه با اندیس |
| `append` | `append(array, value) -> array` | افزودن به انتهای آرایه |

---

## ۲. توابع رجیستری توکار (`src/builtins/`)

### ریاضی، مثلثات و تئوری اعداد (`src/builtins/math.rs` - ۴۲ تابع)
- **ثابت‌ها:** `pi()`, `e()`, `tau()`
- **تبدیل زاویه:** `deg_to_rad(deg)`, `rad_to_deg(rad)`
- **مثلثاتی:** `sin(x)`, `cos(x)`, `tan(x)`, `asin(x)`, `acos(x)`, `atan(x)`, `atan2(y, x)`
- **هایپربولیک:** `sinh(x)`, `cosh(x)`, `tanh(x)`, `asinh(x)`, `acosh(x)`, `atanh(x)`
- **ریشه، توان، لگاریتم:** `ln(x)`, `log2(x)`, `log10(x)`, `exp(x)`, `exp2(x)`, `cbrt(x)`, `hypot(x, y)`
- **اعشار و علامت:** `fract(x)`, `trunc(x)`, `sign(n)`, `sign_float(x)`, `copysign(mag, sign)`
- **بررسی float:** `is_nan(x)`, `is_infinite(x)`, `is_finite(x)`
- **تئوری اعداد:** `gcd(a, b)`, `lcm(a, b)`, `factorial(n)`, `is_prime(n)`, `next_prime(n)`, `is_even(n)`, `is_odd(n)`, `is_between(n, min, max)`
- **تبدیل مبناها:** `to_hex(n)`, `from_hex(s)`, `to_bin(n)`, `from_bin(s)`, `to_oct(n)`, `from_oct(s)`

### متن و رشته (`src/builtins/strings.rs` - ۳۲ تابع)
- **قالب و حالت:** `capitalize(s)`, `title_case(s)`, `pad_left(s, n, fill)`, `pad_right(s, n, fill)`, `center(s, n, fill)`
- **کاراکتر و یونیکد:** `char_at(s, i)`, `code_point(s)`, `from_code_point(code)`, `chars(s)`
- **جستجو و تطابق:** `index_of_char(s, ch)`, `count_char(s, ch)`, `remove_prefix(s, pre)`, `remove_suffix(s, suf)`, `common_prefix(a, b)`, `common_suffix(a, b)`
- **بررسی محتوا:** `is_empty(s)`, `is_ascii(s)`, `is_alphabetic(s)`, `is_numeric(s)`, `is_alphanumeric(s)`, `is_whitespace(s)`, `is_upper(s)`, `is_lower(s)`
- **تحلیل متن:** `words(s)`, `lines(s)`, `levenshtein(s1, s2)` (فاصله ویرایشی کلمات)

### ابزارهای آرایه (`src/builtins/arrays.rs` - ۳۱ تابع)
- **انتخاب و برش:** `first(arr)`, `last(arr)`, `head(arr)`, `tail(arr)`, `take(arr, n)`, `drop(arr, n)`
- **یکتا و مسطح‌سازی:** `unique_int(arr)`, `unique_float(arr)`, `unique_string(arr)`, `flatten_int(arr2d)`, `flatten_float(arr2d)`, `flatten_string(arr2d)`
- **تولید و تقسیم:** `chunk_int(arr, size)`, `chunk_string(arr, size)`, `fill_int(n, val)`, `fill_float(n, val)`, `fill_string(n, val)`, `fill_bool(n, val)`, `compact_string(arr)`
- **جستجو و شمارش:** `index_of_int(arr, val)`, `index_of_float(arr, val)`, `index_of_string(arr, val)`, `last_index_of_int(arr, val)`, `last_index_of_string(arr, val)`, `count_int(arr, val)`, `count_string(arr, val)`
- **ترکیب و جابجایی:** `concat_int(a, b)`, `concat_float(a, b)`, `concat_string(a, b)`, `swap_int(arr, i, j)`, `swap_string(arr, i, j)`

### آمار و محاسبات برداری (`src/builtins/stats.rs` - ۱۰ تابع)
- **ضرب و تمرکز:** `product_int(arr)`, `product_float(arr)`, `geometric_mean(arr)`, `harmonic_mean(arr)`, `mode_int(arr)`
- **درون‌یابی و نرمال‌سازی:** `lerp(a, b, t)`, `normalize(arr)`
- **هندسه و فاصله:** `distance_2d(x1, y1, x2, y2)`, `distance_3d(x1, y1, z1, x2, y2, z2)`, `z_score(val, mean, stdev)`
