# توابع

## تعریف

```persica
def int add(int left, int right) {
    return left + right;
}
```

ساختار: `def`، نوع خروجی، نام، پارامترهای تایپ‌شده و block بدنه.

## فراخوانی

```persica
int result = add(10, 20);
```

تعداد و نوع آرگومان‌ها در semantic analysis بررسی می‌شود. حداکثر ۲۵۵ پارامتر و آرگومان پشتیبانی می‌شود.

## Forward call

همه‌ی signatureهای سطح فایل قبل از بررسی bodyها ثبت می‌شوند؛ بنابراین فراخوانی پیش از declaration معتبر است:

```persica
print(double(21));
def int double(int value) { return value * 2; }
```

## `void`

```persica
def void greet(string name) {
    print("Hello " + name);
    return;
}
```

در تابع `void` می‌توان `return;` نوشت، اما `return value;` خطاست.

## Return-path validation

تابع غیر `void` باید در تمام مسیرهای قابل تشخیص مقدار برگرداند:

```persica
def int choose(bool flag) {
    if (flag) { return 1; }
    return 2;
}
```

## Recursion

```persica
def int factorial(int n) {
    if (n <= 1) { return 1; }
    return n * factorial(n - 1);
}
```

runtime برای جلوگیری از stack overflow منطقی عمق فراخوانی را به ۲۵۶ محدود می‌کند.

## محدودیت‌ها

- function value و closure وجود ندارد.
- overloading و default argument وجود ندارد.
- declaration تابع فقط top-level است.
- mutual recursion به‌دلیل pre-registration signatureها قابل استفاده است.

