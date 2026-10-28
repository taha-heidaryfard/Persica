# متغیرها و Scope

## تعریف

هر متغیر نوع و initializer اجباری دارد:

```persica
int score = 100;
bool active = true;
string name = "Persica";
```

تعریف دوباره‌ی یک نام در همان scope خطاست.

## متغیر `final`

برای اتصالی که نباید دوباره مقداردهی شود، `final` پیش از نوع می‌آید:

```persica
final int max_attempts = 3;
final string product = "Persica";
```

هرگونه `=`, assignment مرکب، `++` یا `--` روی نام final یک semantic error است. در آرایه‌ی final خود binding ثابت است، اما عضوهای آرایه هنوز قابل تغییرند:

```persica
final int[] values = [1, 2];
values[0] = 9;       // مجاز
// values = [3, 4];  // غیرمجاز
```

## Assignment

مقدار جدید باید با نوع declaration برابر باشد:

```persica
int score = 10;
score = 20;
score += 5;
```

assignment راست‌گرا است:

```persica
int first = 0;
int second = 0;
first = second = 9;
```

## Lexical scope

هر block یک scope جدید می‌سازد. lookup از scope داخلی به بیرونی انجام می‌شود:

```persica
int value = 1;
{
    int value = 2;
    print(value); // 2
}
print(value);     // 1
```

این رفتار shadowing نام دارد. خروج از block متغیر داخلی را حذف می‌کند.

## Scope حلقه‌ی `for`

initializer حلقه در scope مخصوص حلقه قرار دارد:

```persica
for (int index = 0; index < 3; index++) {
    print(index);
}
// index اینجا تعریف نشده است.
```

در حلقه‌ی `for ... in` نیز متغیر iteration فقط در scope حلقه وجود دارد و نوع آن از iterable استنتاج می‌شود.

## Scope تابع

هر فراخوانی تابع scope تازه‌ای برای پارامترها و متغیرهای محلی می‌سازد. توابع می‌توانند متغیر global موجود را بخوانند یا assign کنند، مگر اینکه نام محلی آن را shadow کند.

## طول عمر

Persica فعلاً garbage collector جداگانه ندارد. runtime از ownership خود Rust استفاده می‌کند و scopeها پس از خروج آزاد می‌شوند. آرایه‌ها value semantics دارند؛ `append` آرایه‌ی جدید می‌سازد.
