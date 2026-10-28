# ماتریس اعتبارسنجی برای Agents

## دستورات اجباری عمومی

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

## براساس نوع تغییر

| تغییر | بررسی اضافه |
|---|---|
| Lexer | unit test token/span و source Unicode |
| Parser | precedence، missing delimiter و invalid target |
| Semantic | positive + type/scope negative test |
| Runtime | output و value-dependent error |
| Function | arity، recursion، return path و scope |
| Loop | zero iteration، break، continue و nested loop |
| Array | empty، nested، bounds، mutation و type mismatch |
| Built-in | reserved name، arity، type و runtime failure |
| CLI | binary integration، stdout/stderr و exit code |
| Docs-only | link checker و example `--check` |
| Release | release build، version و تمام examples |

## Example validation

```powershell
Get-ChildItem examples -Filter *.persi | ForEach-Object {
    .\target\release\persica.exe --check $_.FullName
}
```

## پایان task

Agent باید در گزارش نهایی مشخص کند:

- چه فایل‌هایی یا رفتارهایی تغییر کردند.
- چه تست‌هایی اجرا شدند.
- آیا breaking change وجود دارد.
- artifact نهایی کجاست.
- چه محدودیت شناخته‌شده‌ای باقی است.

