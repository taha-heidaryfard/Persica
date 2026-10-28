# Persica 0.7 Language Reference

این فایل مرجع فشرده‌ی زبان است. برای توضیح کامل هر بخش، مثال‌ها، edge caseها و محدودیت‌ها از [فهرست مستندات](README.md) و فایل‌های `language/` استفاده کنید.

## Types

| Type | Meaning | Example |
|---|---|---|
| `int` | Signed 64-bit integer | `42` |
| `float` | 64-bit floating-point number | `3.14` |
| `bool` | Boolean | `true` |
| `string` | UTF-8 text | `"Persica"` |
| `T[]` | Array of `T` | `[1, 2, 3]` |
| `void` | Function with no result | `def void log()` |

Arrays can be nested: `int[][] matrix = [[1, 2], [3, 4]];`. Empty arrays receive their type from the declaration, assignment, return type or function parameter.

## Grammar

```ebnf
program       = declaration* EOF ;
declaration   = function | variable | statement ;
function      = "def" type IDENTIFIER "(" parameters? ")" block ;
parameters    = parameter ("," parameter)* ;
parameter     = type IDENTIFIER ;
variable      = "final"? type IDENTIFIER "=" expression ";" ;
type          = ("int" | "float" | "bool" | "string" | "void") ("[" "]")* ;

statement     = block | if | while | for | break | continue | return
              | print | expression ";" ;
block         = "{" declaration* "}" ;
if            = "if" condition statement ("else" statement)? ;
while         = "while" condition statement ;
condition     = expression | "(" expression ")" ;
for           = "for" IDENTIFIER "in" expression statement
              | "for" "(" (variable | expression? ";")
                expression? ";" expression? ")" statement ;
break         = "break" ";" ;
continue      = "continue" ";" ;
return        = "return" expression? ";" ;
print         = "print" "(" print_arguments? ")" ";" ;
print_arguments = expression ("," expression)*
                ("," ("sep" | "end") "=" expression)*
                | ("sep" | "end") "=" expression
                  ("," ("sep" | "end") "=" expression)* ;

expression    = assignment ;
assignment    = logical_or (("=" | "+=" | "-=" | "*=" | "/=" | "%=") assignment)? ;
logical_or    = logical_and ("||" logical_and)* ;
logical_and   = equality ("&&" equality)* ;
equality      = comparison (("==" | "!=") comparison)* ;
comparison    = term ((">" | ">=" | "<" | "<=") term)* ;
term          = factor (("+" | "-") factor)* ;
factor        = unary (("*" | "/" | "%") unary)* ;
unary         = ("!" | "-") unary | postfix ;
postfix       = primary (("(" arguments? ")") | ("[" expression "]") | "++" | "--")* ;
primary       = INTEGER | FLOAT | STRING | "true" | "false" | IDENTIFIER
              | "(" expression ")" | array ;
array         = "[" arguments? "]" ;
arguments     = expression ("," expression)* ;
```

`void` فقط برای نوع خروجی تابع مجاز است. تابع‌ها فقط در سطح اصلی فایل تعریف می‌شوند و حداکثر ۲۵۵ پارامتر یا آرگومان دارند.

## Scopes and control flow

هر block یک scope جدید می‌سازد. متغیر داخلی می‌تواند متغیر بیرونی را shadow کند. اتصال `final` دوباره assign نمی‌شود. initializer حلقه‌ی C-style و متغیر حلقه‌ی `for ... in` فقط داخل همان حلقه قابل دسترسی‌اند.

`break` و `continue` فقط داخل حلقه معتبرند. `return` فقط داخل تابع معتبر است. تابع غیر `void` باید در تمام مسیرهای قابل تشخیص مقدار برگرداند.

## Operators

- `+ - * / %` روی دو `int` یا دو `float` هم‌نوع
- `+` برای اتصال دو `string`
- `< <= > >=` روی دو `int` یا دو `float` هم‌نوع
- `== !=` بین دو مقدار هم‌نوع
- `&& || !` روی `bool`
- `&&` و `||` short-circuit هستند
- arithmetic از overflow محافظت می‌کند؛ تقسیم بر صفر خطای runtime است
- تبدیل ضمنی `int/float` وجود ندارد؛ تبدیل باید صریح باشد

## Arrays

```persica
int[] values = [10, 20, 30];
values[1] = 99;
values[2] += 5;
print(values);
```

index باید `int` نامنفی باشد. دسترسی خارج محدوده یک runtime diagnostic تولید می‌کند. indexing رشته یک `string` تک‌نویسه‌ای برمی‌گرداند.

## Built-in functions

| Function | Result |
|---|---|
| `len(string_or_array)` | تعداد نویسه‌ها یا اعضا |
| `range(end)` / `range(start, end, step)` | آرایه‌ی اعداد با قرارداد Python |
| `append(array, value)` | آرایه‌ی جدید با عضو اضافه‌شده |
| `to_string(value)` | تبدیل مقدار به رشته |
| `to_int(string_or_int)` | تبدیل رشته‌ی معتبر به عدد |
| `to_float(string_or_number)` | تبدیل صریح به float |
| `min(array)` / `max(array)` | کمینه یا بیشینه‌ی آرایه (یا دو عدد) |
| `mean(array)` / `median(array)` | میانگین یا میانه‌ی آرایه‌ی عددی |
| `variance(array)` / `stdev(array)` | واریانس یا انحراف معیار آرایه‌ی عددی |
| `input()` / `input(prompt)` | نمایش prompt اختیاری و خواندن یک خط UTF-8 |
| `read_line()` | نام قدیمی سازگار برای input |

`append` آرایه‌ی ورودی را mutate نمی‌کند؛ نتیجه باید در متغیر ذخیره شود.

فهرست کامل ابزارهای متن، عدد و آرایه در [ابزارهای داده](language/data-tools.md) آمده است.

## Strings, identifiers and comments

escapeهای `\n`، `\r`، `\t`، `\"` و `\\` پشتیبانی می‌شوند. شناسه‌ها Unicode-aware هستند و نام‌هایی مانند `سن` معتبرند.

```persica
// line comment
/* block comment /* nested */ */
```
