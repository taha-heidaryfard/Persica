# مستندات کد و توسعه‌ی پروژه

این بخش برای توسعه‌دهنده‌ای است که می‌خواهد implementation زبان Persica را بفهمد یا تغییر دهد.

## معماری کد

1. [نمای کلی pipeline](compiler/architecture.md)
2. [Lexer](compiler/lexer.md)
3. [Parser](compiler/parser.md)
4. [AST](compiler/ast.md)
5. [Semantic analyzer](compiler/semantic-analysis.md)
6. [Runtime و interpreter](compiler/runtime.md)
7. [Diagnostic system](compiler/diagnostics.md)

## رابط‌ها

- [CLI و قرارداد exit/stdout/stderr](tooling/cli.md)
- [Embedding API برای Rust](tooling/embedding.md)

## توسعه و نگهداری

- [Workflow توسعه‌ی قابلیت](development/workflow.md)
- [استراتژی تست](development/testing.md)
- [سیاست مستندسازی](development/documentation-policy.md)
- [چک‌لیست release](development/release-checklist.md)

## تاریخ نسخه‌ها

- [Persica 0.7.1](releases/v0.7.1.md)
- [Persica 0.7.0](releases/v0.7.0.md)
- [Persica 0.6.0](releases/v0.6.0.md)
- [Persica 0.1.0](releases/v0.1.0.md)
- [Persica 0.5.0](releases/v0.5.0.md)
- [مهاجرت 0.1 به 0.5](releases/migration-0.1-to-0.5.md)
- [قالب release آینده](releases/TEMPLATE.md)

## مرز این بخش

رفتار زبان و راهنمای برنامه‌نویسی برای کاربران در [بخش یادگیری زبان](../learn/README.md) توضیح داده شده است. این دو بخش عمداً مجزا هستند تا جزئیات فنی کامپایلر با یادگیری زبان تداخل نداشته باشد.
