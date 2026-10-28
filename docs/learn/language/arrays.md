# آرایه‌ها

## Declaration و literal

```persica
int[] numbers = [1, 2, 3];
string[] names = ["Ada", "Taha"];
```

تمام اعضا باید هم‌نوع باشند. آرایه‌ی خالی نوع را از context می‌گیرد:

```persica
int[] values = [];
```

literal خالی بدون context معتبر نیست، چون نوع element قابل استنتاج نیست.

## آرایه‌ی چندبعدی

```persica
int[][] matrix = [[1, 2], [3, 4]];
print(matrix[1][0]); // 3
```

طول ردیف‌ها لازم نیست برابر باشد.

## Indexing

```persica
int value = numbers[0];
numbers[1] = 20;
numbers[2] += 5;
```

index باید `int` نامنفی باشد. index منفی یا خارج محدوده runtime diagnostic ایجاد می‌کند.

## Value semantics

آرایه‌ها در assignment و عبور به تابع clone می‌شوند. تغییر آرایه‌ی محلی، آرایه‌ی caller را تغییر نمی‌دهد مگر مقدار جدید return و assign شود.

`append` mutation درجا نیست:

```persica
values = append(values, 4);
```

## Equality و نمایش

برابری آرایه deep است. `print` آرایه را با قالب `[a, b, c]` نمایش می‌دهد و رشته‌های داخل آرایه را داخل quote قرار می‌دهد.

## محدودیت‌های فعلی

- slice و iterator وجود ندارد.
- حذف عضو built-in ندارد.
- array literal باید اعضای صریح داشته باشد؛ spread وجود ندارد.

