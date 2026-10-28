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

مرجع کامل ابزارهای جدید در [ابزارهای متن، عدد و آرایه](data-tools.md) قرار دارد.

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
