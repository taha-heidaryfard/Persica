# قواعد Documentation برای Agents

سیاست کامل انسانی در [documentation-policy](../project/development/documentation-policy.md) قرار دارد. این فایل checklist عملی agent است.

## محل ثبت تغییر

| نوع تغییر | سند لازم |
|---|---|
| syntax/feature | `docs/learn/language/<topic>.md` |
| compiler implementation | `docs/project/compiler/<stage>.md` |
| CLI/API | `docs/project/tooling/` |
| workflow/test | `docs/project/development/` |
| release | `docs/project/releases/vX.Y.Z.md` |
| راهنمای agent | `docs/agents/` و در صورت نیاز `AGENTS.md` |

## قواعد

- سند user-facing نباید جزئیات داخلی غیرضروری داشته باشد.
- سند project باید implementation و invariant واقعی را شرح دهد.
- سند agent باید اجرایی، کوتاه‌مسیر و شامل نام فایل/دستور باشد.
- مثال code باید با نسخه‌ی فعلی معتبر باشد یا صریحاً pseudo-code علامت بخورد.
- link نسبی پس از جابه‌جایی فایل‌ها بررسی شود.
- هر فایل جدید در index همان بخش لینک شود.

## هنگام تغییر نسخه

`Cargo.toml`، `Cargo.lock`، CLI version، CHANGELOG و release document باید هماهنگ باشند.

