# Abstract Syntax Tree

پیاده‌سازی: `src/ast.rs`

AST از enumهای Rust استفاده می‌کند و تمام داده‌های لازم برای semantic analysis و runtime را مالک است.

## Type

`Type` شامل `Int`، `Float`، `Bool`، `String`، `Void` و `Array(Box<Type>)` است. `Box` امکان آرایه‌ی چندبعدی را بدون اندازه‌ی بی‌نهایت enum می‌دهد.

## Expression

`Expr` شامل `kind` و `span` است. variantهای اصلی:

- literal و array literal
- identifier
- assignment و index assignment
- unary و binary
- call
- index
- grouping

Operatorها enum جداگانه دارند؛ runtime و semantic analyzer روی string عملگر تصمیم نمی‌گیرند.

`Literal::Float(f64)` باعث شده AST به‌جای `Eq` فقط `PartialEq` باشد. NaN از source literal ساخته نمی‌شود و built-inهای حساس نتیجه‌ی non-finite را رد می‌کنند.

## Statement

`Stmt` نیز `kind` و `span` دارد:

- variable declaration با پرچم `is_final` و function declaration
- block
- if، while، حلقه‌ی C-style `For` و حلقه‌ی `ForEach`
- break، continue و return
- print با positional arguments و expressionهای اختیاری separator/end، و expression statement

## Function parameter

هر `Parameter` نوع، نام و span خودش را نگه می‌دارد تا duplicate parameter diagnostic دقیق باشد.

## دلیل نگهداری grouping

از نظر محاسباتی grouping می‌تواند حذف شود، اما نگهداری آن span و AST dump را به source نزدیک‌تر می‌کند.

## پایداری API

AST در حال حاضر public است، اما هنوز stability guarantee ندارد. تغییر grammar ممکن است variantها را تغییر دهد؛ مصرف‌کننده‌ی library باید version crate را pin کند.
