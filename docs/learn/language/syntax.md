# Syntax و ساختار برنامه

## فایل source

پسوند پیشنهادی فایل Persica برابر `.persi` است. source باید UTF-8 باشد. برنامه مجموعه‌ای از declaration و statement است و entry point جداگانه ندارد؛ statementهای سطح فایل به‌ترتیب اجرا می‌شوند.

```persica
int value = 10;
print(value);
```

## پایان statement

تعریف متغیر، expression statement، `print`، `return`، `break` و `continue` با `;` پایان می‌یابند. block و declaration تابع پس از `}` semicolon نمی‌خواهند.

## Declarationها

```persica
int age = 20;
final string app_name = "Persica";
string[] names = ["A", "B"];

def int add(int left, int right) {
    return left + right;
}
```

تعریف تابع فقط در سطح اصلی فایل مجاز است. متغیر می‌تواند در سطح فایل، block، حلقه یا تابع تعریف شود.

## Statementها

- block: `{ ... }`
- شرط: `if / else`
- حلقه: `while / for`
- کنترل حلقه: `break / continue`
- خروج از تابع: `return`
- چاپ: `print(value1, value2, sep=" ", end="\n");`
- expression statement، مانند `counter++;`

## Expressionها

Expression مقدار تولید می‌کند. assignment نیز expression است و مقدار جدید را برمی‌گرداند:

```persica
int a = 0;
int b = 0;
a = b = 10;
print(a + b);
```

## محدودیت‌های فعلی

- import و چندفایلی هنوز وجود ندارد.
- function declaration تودرتو مجاز نیست.
- declaration بدون initializer وجود ندارد.
- object، struct، enum و generic هنوز پیاده‌سازی نشده‌اند.

## ورودی و خروجی

```persica
string name = input("Name: ");
print("Hello", name, sep=", ", end="!\n");
```

`print()` نیز معتبر است و فقط newline چاپ می‌کند. آرگومان‌های `sep` و `end` باید رشته باشند و پس از آرگومان‌های عادی قرار بگیرند.
