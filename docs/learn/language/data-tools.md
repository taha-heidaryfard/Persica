# ابزارهای متن، عدد و آرایه

تمام این عملیات مقدار جدید برمی‌گردانند و ورودی را mutate نمی‌کنند. نام آن‌ها برای built-in رزرو است.

## متن و Unicode

| تابع | خروجی | رفتار |
|---|---|---|
| `upper(text)` / `lower(text)` | `string` | تغییر بزرگی حروف |
| `trim(text)` | `string` | حذف whitespace دو طرف |
| `trim_start(text)` / `trim_end(text)` | `string` | حذف whitespace یک طرف |
| `contains(text, part)` | `bool` | وجود زیررشته |
| `starts_with(text, prefix)` / `ends_with(text, suffix)` | `bool` | بررسی ابتدا یا انتها |
| `replace(text, from, to)` | `string` | جایگزینی تمام تطابق‌ها |
| `split(text, separator)` | `string[]` | شکستن متن؛ separator خالی خطاست |
| `join(separator, values)` | `string` | اتصال آرایه‌ی رشته‌ها |
| `repeat(text, count)` | `string` | تکرار؛ count منفی خطاست |
| `find(text, part)` | `int` | index نویسه‌ای اولین تطابق یا `-1` |
| `count(text, part)` | `int` | تعداد تطابق‌های غیرهم‌پوشان |
| `slice(text, start, end?)` | `string` | برش نیمه‌باز با index منفی |
| `reverse(text)` | `string` | معکوس‌کردن نویسه‌ها |

```persica
string[] words = split(trim("  one,two,three  "), ",");
print(join(" | ", words));
print(slice("Persica", 1, -1)); // ersic
print(find("سلام دنیا", "دنیا")); // 5
```

`len`، `find` و `slice` برای رشته برحسب نویسه هستند، نه byteهای UTF-8.

## عدد

| تابع | ورودی و خروجی |
|---|---|
| `abs(value)` | همان `int` یا `float` |
| `min(left, right)` / `max(left, right)` | دو عدد هم‌نوع، همان نوع (یا آرایه‌ی غیرخالی) |
| `clamp(value, low, high)` | سه عدد هم‌نوع، همان نوع |
| `pow(base, exponent)` | دو `float`، نتیجه `float` |
| `sqrt(value)` | `float` نامنفی، نتیجه `float` |
| `round(value)` / `floor(value)` / `ceil(value)` | `float` به `int` |

```persica
print(abs(-12));
print(clamp(120, 0, 100));
print(pow(2.0, 8.0));
print(round(sqrt(20.0)));
```

`clamp` با minimum بزرگ‌تر از maximum و `sqrt` عدد منفی runtime error می‌دهند. خروجی غیرمتناهی `pow` نیز رد می‌شود.

## آرایه

| تابع | رفتار |
|---|---|
| `contains(array, value)` | بررسی وجود عضو |
| `slice(array, start, end?)` | برش با index منفی |
| `reverse(array)` | آرایه‌ی معکوس |
| `sort(array)` | مرتب‌سازی `int[]`، `float[]` یا `string[]` |
| `min(array)` / `max(array)` | یافتن کمینه یا بیشینه در `int[]`، `float[]` یا `string[]` |
| `sum(array)` | جمع `int[]` یا `float[]` غیرخالی |
| `insert(array, index, value)` | آرایه‌ی جدید با عضو اضافه‌شده |
| `remove(array, index)` | آرایه‌ی جدید بدون عضو index |

```persica
int[] values = [3, 1, 2];
print(sort(values));           // [1, 2, 3]
print(min(values), max(values)); // 1 3
print(insert(values, 1, 9));  // [3, 9, 1, 2]
print(remove(values, -1));    // [3, 1]
print(sum(values));           // 6
print(values);                // ورودی تغییر نکرده است
```

نتیجه‌ی عملیاتی که می‌خواهید نگه دارید باید assign شود. `sum`، `min` و `max` آرایه‌ی خالی را رد می‌کنند. در آرایه‌های رشته‌ای، `min` کوتاه‌ترین رشته و `max` بلندترین رشته را برحسب تعداد نویسه‌های Unicode برمی‌گردانند (در صورت برابری طول، ترتیب الفبایی مبنای tie-break است).

## آمار

| تابع | ورودی | خروجی | رفتار |
|---|---|---|---|
| `mean(array)` | `int[]` یا `float[]` | `float` | میانگین حسابی اعضا |
| `median(array)` | `int[]` یا `float[]` | `float` | میانه‌ی داده‌ها |
| `variance(array)` | `int[]` یا `float[]` | `float` | واریانس جامعه ($\sigma^2$) |
| `stdev(array)` | `int[]` یا `float[]` | `float` | انحراف معیار جامعه ($\sigma$) |

```persica
int[] data = [2, 4, 4, 4, 5, 5, 7, 9];
print(mean(data));     // 5.0
print(median(data));   // 4.5
print(variance(data)); // 4.0
print(stdev(data));    // 2.0
```

توابع آماری روی آرایه‌ی خالی خطای زمان اجرا تولید می‌کنند.
