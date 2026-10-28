# Semantic Analyzer

پیاده‌سازی: `src/semantic.rs`

Semantic pass AST را بدون اجرا اعتبارسنجی می‌کند.

## دو مرحله‌ی تابع

1. `collect_functions` تمام signatureهای top-level را ثبت می‌کند.
2. statementها و bodyها بررسی می‌شوند.

این طراحی forward call و mutual recursion را ممکن می‌کند.

## Symbol table

scopeها به‌شکل `Vec<HashMap<String, VariableSymbol>>` نگهداری می‌شوند. هر symbol نوع و وضعیت `is_final` را دارد. تعریف در آخرین map و lookup از انتها به ابتدا انجام می‌شود؛ assignment روی final رد می‌شود.

تابع‌ها namespace جدا دارند. built-inها رزرو هستند و declaration هم‌نام رد می‌شود.

## Type inference expression

هر expression یک `Type` تولید می‌کند. binary operation با type دو operand بررسی می‌شود. assignment نوع binding را حفظ می‌کند.

آرایه‌ی خالی تنها با expected type معتبر است. helper `expression_type_as` context declaration، assignment، return یا argument را اعمال می‌کند.

`float` نوع عددی مستقل است و تبدیل ضمنی با `int` ندارد. عملگرهای حسابی و مقایسه‌ای دو operand هم‌نوع می‌خواهند. built-inهای 0.7 در `call_type` به‌صورت overload محدود اعتبارسنجی می‌شوند و توابع generic آرایه element type را حفظ می‌کنند.

در `ForEach`، iterable باید آرایه یا رشته باشد و نوع متغیر iteration از element type یا `string` استنتاج می‌شود. `print` تمام مقدارها را non-void و `sep/end` را string می‌خواهد. `input` صفر یا یک prompt رشته‌ای و `range` یک تا سه عدد می‌پذیرند.

## Control-flow context

- `loop_depth` اعتبار break/continue را تعیین می‌کند.
- `current_return` اعتبار return و نوع آن را تعیین می‌کند.
- ورود و خروج loop حتی در صورت خطا depth را restore می‌کند.

## Return analysis

تابع غیر void باید مسیر تضمین‌شده‌ی return داشته باشد. analyzer block و `if` دارای دو شاخه را بررسی می‌کند. این تحلیل محافظه‌کار است و حلقه‌ی ظاهراً بی‌نهایت را تضمین return نمی‌داند.

## مواردی که semantic نیستند

تقسیم بر صفر، index خارج محدوده و parsing رشته به int به مقدار runtime وابسته‌اند و در این مرحله بررسی نمی‌شوند.

## افزودن قابلیت جدید

هر AST variant جدید باید در `statement` یا `expression_type` پوشش داده شود. exhaustiveness compiler Rust کمک می‌کند هیچ case فراموش نشود.
