# مهاجرت از Persica 0.1 به 0.5

## سازگاری عمومی

syntaxهای متغیر، expression، `print`، string و comment بدون تغییر باقی مانده‌اند. بیشتر برنامه‌های 0.1 بدون ویرایش اجرا می‌شوند.

## Keywordهای جدید

این نام‌ها در 0.5 رزرو شده‌اند و اگر در 0.1 identifier بودند باید rename شوند:

```text
void def return if else while for break continue
```

نام built-inهای زیر نیز برای function declaration کاربر رزرو شده‌اند:

```text
len range append to_string to_int read_line
```

## استفاده از قابلیت‌های جدید

### جایگزینی repetition دستی

```persica
for (int i = 0; i < 10; i++) {
    print(i);
}
```

### استخراج منطق به تابع

```persica
def int calculate(int value) {
    return value * 2;
}
```

### collection

```persica
int[] values = range(0, 10);
values = append(values, 10);
```

## تغییر CLI

گزینه‌های قبلی باقی مانده‌اند. برای CI از `--check` استفاده کنید. برای snippet از `--eval` و برای pipeline از فایل `-` استفاده کنید.

## Runtime input

`persica::run` ورودی را غیرفعال نگه می‌دارد. hostهایی که `read_line` می‌خواهند باید از `Interpreter::with_input` یا `Interpreter::with_stdin` استفاده کنند.

## پیشنهاد اعتبارسنجی مهاجرت

```powershell
persica --check old-program.persi
persica old-program.persi
```

مرحله‌ی اول خطاهای syntax/type و مرحله‌ی دوم خطاهای value-dependent را آشکار می‌کند.

