# رشته‌ها، Unicode و Comment

## رشته

رشته با double quote نوشته می‌شود:

```persica
string text = "Persica";
```

رشته‌ی معمولی نمی‌تواند مستقیم وارد خط بعد شود.

## Escapeها

| Escape | مقدار |
|---|---|
| `\n` | newline |
| `\r` | carriage return |
| `\t` | tab |
| `\"` | double quote |
| `\\` | backslash |

escape ناشناخته lexer error است.

## Unicode

source UTF-8 است. identifier با حرف Unicode یا `_` آغاز می‌شود و سپس می‌تواند حرف یا عدد Unicode داشته باشد:

```persica
int سن = 20;
string پیام = "سلام";
print(پیام);
```

عدد literal فعلاً فقط digitهای ASCII را قبول می‌کند.

## String indexing

index براساس نویسه است، نه byte:

```persica
print("سلام"[0]); // س
```

## Line comment

```persica
// تا انتهای خط نادیده گرفته می‌شود.
```

## Block comment

```persica
/* چند خط
   comment */
```

Block comment می‌تواند تودرتو باشد:

```persica
/* outer /* inner */ outer */
```

