# عملگرها و تقدم

جدول از کمترین به بیشترین تقدم:

| سطح | عملگر | associativity |
|---|---|---|
| assignment | `= += -= *= /= %=` | راست |
| logical OR | `||` | چپ |
| logical AND | `&&` | چپ |
| equality | `== !=` | چپ |
| comparison | `< <= > >=` | چپ |
| additive | `+ -` | چپ |
| multiplicative | `* / %` | چپ |
| unary | `! -` | راست |
| postfix | call، index، `++ --` | چپ |

## Arithmetic

`+ - * / %` روی دو `int` کار می‌کنند. `+` روی دو `string` آن‌ها را متصل می‌کند. ترکیب `string` و `int` مجاز نیست؛ از `to_string` استفاده کنید.

## Comparison و equality

`< <= > >=` فقط برای `int` هستند. `==` و `!=` دو مقدار هم‌نوع را مقایسه می‌کنند و برای آرایه نیز deep equality دارند.

## Logical

`!`، `&&` و `||` فقط `bool` می‌پذیرند. `&&` و `||` short-circuit هستند:

```persica
bool safe = false && (10 / 0 == 1); // خطای runtime رخ نمی‌دهد.
```

## Compound assignment

```persica
int value = 10;
value += 2;
value *= 3;
value++;
```

روی عضو آرایه نیز قابل استفاده است:

```persica
int[] values = [1, 2];
values[0] += 9;
```

`++` و `--` در نسخه‌ی فعلی مقدار جدید را تولید می‌کنند و فقط روی variable ساده مجازند.

