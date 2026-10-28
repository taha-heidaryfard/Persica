# سیستم نوع

Persica در مرحله‌ی semantic به‌صورت statically typed بررسی می‌شود. نوع متغیر، پارامتر و خروجی تابع صریح است.

## `int`

عدد صحیح signed 64-bit در بازه‌ی `i64`:

```persica
int count = 42;
int negative = -10;
```

literal خارج از بازه در parser رد می‌شود. overflow عملگرها و تقسیم بر صفر runtime diagnostic هستند.

## `float`

عدد اعشاری ۶۴ بیتی است و literal آن نقطه و حداقل یک رقم پس از نقطه دارد:

```persica
float price = 12.5;
float ratio = 1.0 / 3.0;
float[] samples = [1.5, 2.0, 2.75];
```

تبدیل ضمنی عددی وجود ندارد؛ `1 + 2.0` خطاست و شکل صریح آن `to_float(1) + 2.0` است. تقسیم یا remainder بر `0.0` خطای runtime است.

## `bool`

```persica
bool enabled = true;
bool done = false;
```

شرط‌های `if`، `while` و `for` باید `bool` باشند.

## `string`

رشته‌ی UTF-8:

```persica
string message = "سلام";
```

`len` تعداد نویسه‌های Unicode را برمی‌گرداند، نه تعداد byteها. indexing نیز یک نویسه را به‌شکل `string` برمی‌گرداند.

## `void`

فقط به‌عنوان نوع خروجی تابع مجاز است:

```persica
def void log(string message) {
    print(message);
}
```

متغیر، پارامتر یا آرایه‌ی `void` مجاز نیست. مقدار `void` قابل چاپ یا مقایسه نیست.

## آرایه

`T[]` آرایه‌ی همگن از `T` است:

```persica
int[] numbers = [1, 2, 3];
string[][] table = [["a"], ["b", "c"]];
```

اعضای array literal باید هم‌نوع باشند. نوع آرایه‌ی خالی از context گرفته می‌شود:

```persica
int[] values = [];
```

## دیکشنری و مپ (`map<K, V>` و `dict`)

پرسیکا از ساختارهای داده کلید-مقدار (Map / Dictionary) پشتیبانی می‌کند.

### مپ با نوع مشخص (`map<K, V>`)
```persica
map<string, int> scores = {"ali": 20, "reza": 18};
scores["sara"] = 19;
print(scores["ali"]); // 20
```

### دیکشنری آزاد به سبک پایتون (`dict` یا `map<any, any>`)
نوع `dict` میان‌بری برای `map<any, any>` است که به شما امکان نگه‌داری هر نوع کلیدی با هر نوع مقداری را می‌دهد:
```persica
dict user = {
    "name": "طاها",
    "age": 22,
    1: "رتبه اول"
};
print(user[1]); // رتبه اول
```

### متدهای کمکی مپ:
- `len(m)`: تعداد اعضا
- `contains_key(m, key)`: بررسی وجود کلید
- `keys(m)`: آرایه‌ی کلیدها
- `values(m)`: آرایه‌ی مقادیر
- `remove_key(m, key)`: ایجاد دیکشنری جدید بدون کلید مشخص‌شده

## سازگاری نوع

تبدیل ضمنی وجود ندارد. این کد خطاست:

```persica
int value = "42";
```

تبدیل باید صریح باشد:

```persica
int value = to_int("42");
string text = to_string(value);
```
