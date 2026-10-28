# کنترل جریان

## `if / else`

```persica
if age >= 18 {
    print("adult");
} else {
    print("minor");
}
```

شرط باید `bool` باشد. `else if` از قرارگرفتن یک `if` داخل شاخه‌ی `else` ساخته می‌شود.

## `while`

```persica
int count = 0;
while count < 3 {
    print(count);
    count++;
}
```

شرط پیش از هر iteration بررسی می‌شود.

پرانتز دور شرط اختیاری است؛ شکل قدیمی `while (condition)` نیز معتبر می‌ماند.

## `for ... in`

شکل اصلی و ساده‌ی iteration روی آرایه یا رشته:

```persica
for number in range(5) {
    print(number);
}

for letter in "سلام" {
    print(letter);
}
```

نوع متغیر حلقه از اعضای iterable استنتاج می‌شود و فقط داخل حلقه scope دارد.

## `for` به سبک C (سازگار)

```persica
for (int index = 0; index < 10; index++) {
    print(index);
}
```

هر سه بخش اختیاری‌اند:

```persica
for (;;) {
    break;
}
```

ترتیب اجرا: initializer یک‌بار، condition، body، increment و سپس condition بعدی.

## `break`

نزدیک‌ترین حلقه را تمام می‌کند. بیرون حلقه semantic error است.

## `continue`

iteration فعلی را متوقف می‌کند. در `for`، increment همچنان پیش از condition بعدی اجرا می‌شود. بیرون حلقه semantic error است.

## Block توصیه‌شده

بدنه‌ی تک‌statement بدون `{}` مجاز است، اما برای scope واضح و جلوگیری از خطا استفاده از block توصیه می‌شود.
