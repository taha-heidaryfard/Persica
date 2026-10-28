# مستندات Persica

مستندات Persica براساس مخاطب به سه بخش مستقل تقسیم شده است. از بخشی شروع کنید که با هدف شما هماهنگ است.

## ۱. یادگیری خود زبان

[ورود به بخش Learn](learn/README.md)

برای کسی که می‌خواهد با Persica برنامه بنویسد:

- نصب و اولین برنامه
- syntax و typeها
- متغیر، scope و operator
- شرط و حلقه
- تابع و recursion
- آرایه و built-in
- خطاها و debugging

این بخش درباره‌ی رفتار زبان است و نیاز به دانش Rust ندارد.

## ۲. راهنمای Agents

[ورود به بخش Agents](agents/README.md)

برای agentهای هوش مصنوعی و ابزارهای خودکار که repository را تغییر می‌دهند:

- نقشه‌ی سریع فایل‌ها و pipeline
- invariantهای compiler/runtime
- playbook افزودن قابلیت یا رفع bug
- ماتریس تست براساس نوع تغییر
- قوانین documentation و release

فایل [AGENTS.md](../AGENTS.md) در ریشه نیز entry point خودکار agentهاست.

## ۳. کد و معماری پروژه

[ورود به بخش Project](project/README.md)

برای توسعه‌دهنده‌ی compiler و maintainer پروژه:

- معماری Lexer، Parser، AST و Semantic Analyzer
- Runtime، diagnostics و embedding API
- CLI و قرارداد I/O
- workflow، testing و release process
- تاریخ نسخه‌ها و migration

## منابع سطح ریشه

- [README پروژه](../README.md)
- [CHANGELOG](../CHANGELOG.md)
- [ROADMAP](../ROADMAP.md)
- [LICENSE](../LICENSE)

## قانون منبع حقیقت

- رفتار زبان: `docs/learn/`
- روش پیاده‌سازی: `docs/project/`
- دستورالعمل عملیات agent: `docs/agents/`
- رفتار اثبات‌شده: `tests/`
- وضعیت نسخه: `Cargo.toml`

