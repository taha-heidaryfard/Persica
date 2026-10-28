# توابع داخلی

نام built-inها رزرو است و نمی‌توان تابع کاربر با همان نام تعریف کرد.

## `len(value) -> int`

برای `string` تعداد نویسه‌های Unicode و برای array تعداد اعضا را برمی‌گرداند.

```persica
print(len("سلام"));   // 4
print(len([1, 2]));  // 2
```

## `range(end)`, `range(start, end)`, `range(start, end, step) -> int[]`

همان قرارداد بازه‌ی Python را دارد؛ `end` داخل نتیجه نیست:

```persica
int[] values = range(2, 5); // [2, 3, 4]
int[] first = range(4); // [0, 1, 2, 3]
int[] down = range(5, 0, -2); // [5, 3, 1]
```

گام صفر runtime error است. جهت ناسازگار با گام نتیجه‌ی خالی می‌دهد. برای کنترل مصرف حافظه، بیش از ۱٬۰۰۰٬۰۰۰ عضو مجاز نیست.

## `append(array, value) -> array`

آرایه‌ی جدیدی با عضو اضافه‌شده برمی‌گرداند. نوع value باید با element type برابر باشد.

```persica
values = append(values, 10);
```

## `to_string(value) -> string`

مقدار غیر `void` را با همان قالبی که `print` استفاده می‌کند به string تبدیل می‌کند.

## `to_int(value) -> int`

`int` را بدون تغییر، رشته‌ی decimal معتبر یا float بدون بخش کسری را به عدد تبدیل می‌کند. مقدار نامعتبر یا خارج محدوده runtime diagnostic می‌دهد.

```persica
int answer = to_int("42");
```

## `to_float(value) -> float`

`float` را بدون تغییر و `int` یا رشته‌ی اعشاری معتبر را به float تبدیل می‌کند:

```persica
float price = to_float("12.50");
float half = to_float(1) / 2.0;
```

## توابع رجیستری توکار (Built-in Registry)

از نسخه‌ی ۰.۸ به بعد، پرسیکا دارای رجیستری توکار یکپارچه با بیش از ۱۱۵ تابع آماده است. این توابع به عنوان کتابخانه‌ی پیش‌فرض (Prelude) در دسترس هستند و در صورت نیاز توسط توابع کاربر قابل بازتعریف (Shadowing) می‌باشند.

### ۱. ریاضیات و مثلثات
- **ثابت‌ها:** `pi()`, `e()`, `tau()`
- **تبدیل زاویه:** `deg_to_rad(deg)`, `rad_to_deg(rad)`
- **مثلثاتی:** `sin(x)`, `cos(x)`, `tan(x)`, `asin(x)`, `acos(x)`, `atan(x)`, `atan2(y, x)`
- **هایپربولیک:** `sinh(x)`, `cosh(x)`, `tanh(x)`, `asinh(x)`, `acosh(x)`, `atanh(x)`
- **توان، لگاریتم و ریشه:** `ln(x)`, `log2(x)`, `log10(x)`, `exp(x)`, `exp2(x)`, `cbrt(x)`, `hypot(x, y)`
- **اعشار و علامت:** `fract(x)`, `trunc(x)`, `sign(n)`, `sign_float(x)`, `copysign(mag, sign)`
- **بررسی float:** `is_nan(x)`, `is_infinite(x)`, `is_finite(x)`
- **تئوری اعداد:** `gcd(a, b)`, `lcm(a, b)`, `factorial(n)` (تا ۲۰), `is_prime(n)`, `next_prime(n)`, `is_even(n)`, `is_odd(n)`, `is_between(n, min, max)`
- **مبناها:** `to_hex(n)`, `from_hex(s)`, `to_bin(n)`, `from_bin(s)`, `to_oct(n)`, `from_oct(s)`

### ۲. رشته و متن
- **تغییر حالت و قالب:** `capitalize(s)`, `title_case(s)`, `pad_left(s, len, fill)`, `pad_right(s, len, fill)`, `center(s, len, fill)`
- **کاراکتر و یونیکد:** `char_at(s, i)`, `code_point(s)`, `from_code_point(code)`, `chars(s)`
- **جستجو و تطابق:** `index_of_char(s, ch)`, `count_char(s, ch)`, `remove_prefix(s, pre)`, `remove_suffix(s, suf)`, `common_prefix(a, b)`, `common_suffix(a, b)`
- **بررسی نوع کاراکترها:** `is_empty(s)`, `is_ascii(s)`, `is_alphabetic(s)`, `is_numeric(s)`, `is_alphanumeric(s)`, `is_whitespace(s)`, `is_upper(s)`, `is_lower(s)`
- **تحلیل متن:** `words(s)`, `lines(s)`, `levenshtein(s1, s2)`

### ۳. کار با آرایه‌ها
- **انتخاب و پیمایش:** `first(arr)`, `last(arr)`, `head(arr)`, `tail(arr)`, `take(arr, n)`, `drop(arr, n)`
- **یکتا و مسطح‌سازی:** `unique_int(arr)`, `unique_float(arr)`, `unique_string(arr)`, `flatten_int(arr2d)`, `flatten_float(arr2d)`, `flatten_string(arr2d)`
- **دسته‌بندی و تولید:** `chunk_int(arr, size)`, `chunk_string(arr, size)`, `fill_int(n, val)`, `fill_float(n, val)`, `fill_string(n, val)`, `fill_bool(n, val)`, `compact_string(arr)`
- **جستجو و شمارش در آرایه:** `index_of_int(arr, val)`, `index_of_float(arr, val)`, `index_of_string(arr, val)`, `last_index_of_int(arr, val)`, `last_index_of_string(arr, val)`, `count_int(arr, val)`, `count_string(arr, val)`
- **ترکیب و جابجایی:** `concat_int(a, b)`, `concat_float(a, b)`, `concat_string(a, b)`, `swap_int(arr, i, j)`, `swap_string(arr, i, j)`

### ۴. آمار و هندسه
- **ضرب و میانگین‌ها:** `product_int(arr)`, `product_float(arr)`, `geometric_mean(arr)`, `harmonic_mean(arr)`, `mode_int(arr)`
- **درون‌یابی و نرمال‌سازی:** `lerp(a, b, t)`, `normalize(arr)`
- **فاصله و استانداردسازی:** `distance_2d(x1, y1, x2, y2)`, `distance_3d(x1, y1, z1, x2, y2, z2)`, `z_score(val, mean, stdev)`


## `input()`, `input(prompt) -> string`

یک خط از ورودی host می‌خواند و `CR/LF` انتهایی را حذف می‌کند. اگر prompt داده شود، پیش از خواندن بدون newline چاپ می‌شود.

```persica
string name = input("Name: ");
int age = to_int(input("Age: "));
```

هنگام embedding باید interpreter با input مناسب ساخته شود؛ در غیر این صورت runtime diagnostic ایجاد می‌شود.

## `read_line() -> string`

alias سازگار نسخه‌های قبلی برای `input()` است. برای کد جدید `input` توصیه می‌شود.

## `print(..., sep=" ", end="\n")`

`print` statement است، نه تابع مقدارده. صفر یا چند مقدار می‌گیرد، آن‌ها را با `sep` جدا می‌کند و در پایان `end` را می‌نویسد:

```persica
print("A", 2, true);                 // A 2 true
print("left", "right", sep=" | "); // left | right
print("loading", end="...");
print();
```
