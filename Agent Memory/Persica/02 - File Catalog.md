# 02 - File Catalog 📂

کاتالوگ کامل و دقیق تمام فایل‌های موجود در ریپازیتوری Persica، نقش و مسئولیت‌های هر کدام.

---

## 🗂️ ریشه پروژه (`/`)

| فایل | نقش و وظیفه |
| :--- | :--- |
| `Cargo.toml` | پیکربندی پکیج راست، نسخه پکیج، فلگ‌ها و پروفایل‌ها (فاقد هرگونه dependency خارجی). |
| `AGENTS.md` | دستورالعمل‌های اجباری و خط قرمزهای تمام agentها به زبان فارسی. |
| `README.md` | معرفی پروژه و راهنمای راه‌اندازی و استفاده برای عموم. |
| `ROADMAP.md` | نقشه راه نسخه‌های گذشته، حال و آینده پرسیکا. |

---

## ⚙️ پیاده‌سازی اصلی (`src/`)

### ریشه `src/`
- **`src/lib.rs`**: ریشه کتابخانه. تعریف ماژول‌های عمومی (`lexer`, `parser`, `semantic`, `runtime`, `builtins`, `ast`, `diagnostic`, `source`). متدهای عمومی `compile` و `run`.
- **`src/main.rs`**: فایل اجرایی CLI. پردازش آرگومان‌های خط فرمان (`--check`, `--tokens`, `--ast`, `-e`, خواندن از فایل یا stdin).
- **`src/ast.rs`**: تعاریف اصلی گره‌های AST. شامل `Type` (با پشتیبانی از `Any`, `Map`), `Expr`, `ExprKind` (شامل `Map`), `Stmt`, `StmtKind`, `Literal`, عملگرها.
- **`src/diagnostic.rs`**: ساختار گزارش خطا. `Diagnostic`, `Phase` (Lexer, Parser, Semantic, Runtime).
- **`src/source.rs`**: مدیریت منبع متنی، بازه‌ها (`Span`)، شماره سطر و ستون خطاهای انسانی.

### ۱. ماژول لغوی (`src/lexer/`)
- **`src/lexer/mod.rs`**: منطق اسکنر و متد `tokenize(source)`. اسکن کاراکترها، رشته‌ها، اعداد، کامنت‌ها و کلمات کلیدی.
- **`src/lexer/token.rs`**: ساختار `Token` و شناسه `TokenKind` (از جمله `Map`, `Dict`, `Any`, `Colon`).

### ۲. ماژول تجزیه (`src/parser/`)
- **`src/parser/mod.rs`**: ساختار `Parser`، متد `parse`، ابزارهای کمکی مصرف توکن (`consume`, `matches`, `check`, `is_type_start`).
- **`src/parser/stmt.rs`**: پارس دستورات (`StmtKind`): متغیرها، توابع، حلقه‌های `if`, `while`, `for`, `for-in` (با یا بدون پرانتز)، `print` با آرگومان‌های نام‌دار (`sep`, `end`) و متد `parse_type`.
- **`src/parser/expr.rs`**: پارس عبارات و اولویت عملگرها: انتساب، انتساب ایندکس (`m[k] = v`)، عملگرهای بولی، مقایسه‌ای، محاسباتی، فراخوانی تابع، ایندکس و لیترال‌های مپ `{k: v}` و آرایه `[v]`.

### ۳. ماژول تحلیل معنایی (`src/semantic/`)
- **`src/semantic/mod.rs`**: ساختار `Analyzer` و متد `validate`. مدیریت پشته Scope و جمع‌آوری اولیه‌ی امضای تمام توابع (`collect_functions`).
- **`src/semantic/symbol.rs`**: ساختارهای نمادها: `VariableSymbol`, `FunctionSignature` و توابع ساخت خطای معنایی.
- **`src/semantic/expr.rs`**: بررسی نوع عبارات، انتساب ایندکس مپ و آرایه، استنتاج نوع مپ‌های خالی و متد حیاتی `is_assignable` و `require_type`.
- **`src/semantic/stmt.rs`**: بررسی نوع دستورات، شرط‌ها، اعتبارسنجی `return`، عمق حلقه برای `break`/`continue`، و استخراج نوع عنصر در `for-in`.
- **`src/semantic/builtins.rs`**: مدیریت توابع رزرو شده، بررسی آرگومان‌های توابع داخلی خاص (`len`, `contains_key`, `keys`, `values`, `remove_key`) و اتصال خودکار به رجیستری توکار.

### ۴. ماژول زمان اجرا (`src/runtime/`)
- **`src/runtime/mod.rs`**: تعریف `Interpreter`, `Binding`, `Function`, `Flow` (Normal, Break, Continue, Return), `Input`.
- **`src/runtime/value.rs`**: ساختار مقادیر زمان اجرا `Value` (`Int`, `Float`, `Bool`, `String`, `Array`, `Map`, `Void`) و متد بررسی انطباق `is_assignable_to`.
- **`src/runtime/eval.rs`**: ارزیابی عبارات و اجرای دستورات: حلقه‌ها، انتساب ایندکس مپ و آرایه (`assign_index`), پیمایش مپ با `for-in`.
- **`src/runtime/builtins.rs`**: پیاده‌سازی اجرای توابع وضعیت‌دار و رزرو: `len`, `input`, `read_line`, `range`, `contains_key`, `keys`, `values`, `remove_key`.

### ۵. ماژول رجیستری توکار (`src/builtins/`)
- **`src/builtins/mod.rs`**: رجیستری مرکزی با `LazyLock<HashMap>`، ساختار `Builtin`، تایپ‌های `ParamType` و `ReturnType`.
- **`src/builtins/math.rs`**: ۴۲ تابع ریاضیاتی، مثلثاتی، هایپربولیک، تئوری اعداد و تبدیل مبناها.
- **`src/builtins/strings.rs`**: ۳۲ تابع دستکاری و تحلیل متن، یونیکد و فاصله لون‌اشتاین.
- **`src/builtins/arrays.rs`**: ۳۱ تابع ابزارهای آرایه‌ها: انتخاب، اسلایس، مسطح‌سازی، تقسیم تکه‌ای و جابجایی.
- **`src/builtins/stats.rs`**: ۱۰ تابع آماری، میانگین‌ها، درون‌یابی و محاسبات برداری.

---

## 🧪 آزمون‌ها (`tests/`)

| فایل تست | موضوع تست |
| :--- | :--- |
| `tests/cli.rs` | تست آرگومان‌های CLI، ارزیابی اینلاین (`-e`)، بررسی صرف (`--check`) و ورودی از stdin. |
| `tests/language.rs` | تست هسته زبان: عملگرها، انواع داده اولیه، کنترل سرریز، اتصال متغیرها. |
| `tests/version_05.rs` | تست ویژگی‌های نسخه ۰.۵: توابع کاربر، بازگشت مقدار، توابع بازگشتی، آرایه‌ها و اسلایس. |
| `tests/version_06.rs` | تست ویژگی‌های نسخه ۰.۶: متغیرهای `final`، حلقه‌های `for-in`، تابع `range`، پرینت پیشرفته. |
| `tests/version_07.rs` | تست ویژگی‌های نسخه ۰.۷: اعداد ممیز شناور `float`، تبدیل‌های عددی صریح، ابزارهای متن. |
| `tests/version_07_1.rs` | تست ویژگی‌های نسخه ۰.۷.۱: اکسترمم‌های آرایه (`min`/`max`) و توابع آماری پایه. |
| `tests/builtins_registry.rs` | تست جامع سیستم رجیستری و بیش از ۱۱۵ تابع ریاضی، متن، آرایه و آمار. |
| `tests/version_08.rs` | تست جامع ویژگی‌های نسخه ۰.۸: `map<K, V>`، دیکشنری پایتونی `dict`، متدهای کمکی و پیمایش. |

---

## 💡 نمونه کدهای اجرایی (`examples/`)

- `examples/features.persi`: نمونه‌های ساده اولیه زبان.
- `examples/fibonacci.persi`: محاسبه دنباله فیبوناچی با تابع بازگشتی.
- `examples/fizzbuzz.persi`: الگوریتم کلاسیک FizzBuzz.
- `examples/hello_input.persi`: دریافت ورودی کاربر با `input()`.
- `examples/syntax_06.persi`: سینتکس مدرن نسخه‌های میانی.
- `examples/data_tools_07.persi`: استفاده از توابع متنی و آرایه‌ای نسخه ۰.۷.
- `examples/statistics_07_1.persi`: محاسبات آماری نسخه ۰.۷.۱.
- `examples/registry_gallery_08.persi`: ویترین جامع بیش از ۱۰۰ تابع رجیستری توکار.
- `examples/map_and_dict_08.persi`: ویترین جامع دیکشنری و مپ (`dict`, `map<string, int>`, `map<any, any>`).
