# Runtime و Interpreter

پیاده‌سازی: `src/runtime.rs`

Runtime یک tree-walk interpreter است که `Program` اعتبارسنجی‌شده را اجرا می‌کند.

## Value

مقادیر runtime:

- `Int(i64)`
- `Float(f64)`
- `Bool(bool)`
- `String(String)`
- `Array(Vec<Value>)`
- `Void`

`Binding` علاوه بر value نوع declaration و `is_final` را نگه می‌دارد تا runtime نیز assignment نامعتبر را دفاعی رد کند.

## Environment

مانند semantic analyzer، runtime stack از scopeها دارد. lookup و assignment از داخلی‌ترین scope به بیرون انجام می‌شود.

## Control flow

اجرای statement نتیجه‌ی داخلی `Flow` تولید می‌کند:

- `Normal`
- `Break`
- `Continue`
- `Return(Value)`

Block و loop این flow را propagate یا consume می‌کنند. این روش از exception برای کنترل جریان استفاده نمی‌کند.

## Functions

پیش از اجرای top-level، declaration تابع‌ها در map جمع می‌شوند. call آرگومان‌ها را در scope caller evaluate، scope جدید می‌سازد، پارامترها را bind و body را اجرا می‌کند.

عمق call به ۲۵۶ محدود است. این محدودیت diagnostic کنترل‌شده را جایگزین stack overflow نامشخص می‌کند.

## Arrays

آرایه‌ها value semantics دارند. index assignment مسیر array تودرتو را تا binding ریشه پیدا و عضو را mutate می‌کند. bounds و index منفی در runtime بررسی می‌شوند.

## I/O

هر فراخوانی `print` برای API سازگار `interpret` در `Vec<String>` ثبت می‌شود. هم‌زمان متن دقیق stdout شامل `sep`، `end`، prompt و newlineهای داخلی در یک buffer ساخته می‌شود و `interpret_text` آن را برمی‌گرداند. `with_stdio` خروجی را همان لحظه stream و flush می‌کند. ورودی سه حالت دارد:

- disabled در `Interpreter::new`
- صف قابل تزریق در `Interpreter::with_input`
- stdin در `Interpreter::with_stdin`

`input(prompt)` ابتدا prompt اختیاری را emit و سپس خط را می‌خواند. `ForEach` آرایه را عضو‌به‌عضو و رشته را نویسه‌به‌نویسه اجرا می‌کند. `range` گام مثبت و منفی دارد و گام صفر را رد می‌کند.

این جداسازی تست deterministic را ممکن می‌کند.

## Built-in dispatch

Built-inها پیش از user function در `call` dispatch می‌شوند. semantic analyzer arity/type را بررسی می‌کند و runtime خطاهای value-dependent را مدیریت می‌کند.

Built-inهای داده‌ی 0.7 value semantics دارند. عملیات رشته Unicode-aware است، slice از index منفی پشتیبانی می‌کند، مرتب‌سازی float از total ordering استفاده می‌کند و جمع int overflow را diagnostic می‌کند. تکرار رشته به خروجی ۱۰ مگابایتی محدود است.
