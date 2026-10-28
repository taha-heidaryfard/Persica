# اولین برنامه‌ی Persica

یک فایل به نام `hello.persi` بسازید:

```persica
string name = "Persica";
int year = 2026;

print("Hello " + name);
print("year=" + to_string(year));
```

اجرا:

```powershell
persica hello.persi
```

خروجی:

```text
Hello Persica
year=2026
```

## برنامه‌ی تعاملی

```persica
string name = input("What is your name? ");
print("Hello", name, sep=", ", end="!\n");
```

`input` prompt را نمایش می‌دهد، یک خط از stdin می‌خواند و newline انتهایی را حذف می‌کند.

## تابع و حلقه

```persica
def int square(int value) {
    return value * value;
}

for number in range(1, 6) {
    print(square(number));
}
```

## آرایه

```persica
int[] values = [10, 20, 30];
values[1] += 5;
values = append(values, 40);

print(values);
print(len(values));
```

خروجی:

```text
[10, 25, 30, 40]
4
```

## خطای نمونه

```persica
int age = true;
```

این برنامه قبل از اجرا در مرحله‌ی semantic رد می‌شود؛ چون مقدار `bool` به متغیر `int` داده شده است. Persica نام فایل، خط، ستون و بخش خطادار را نمایش می‌دهد.
