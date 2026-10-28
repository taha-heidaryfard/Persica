# Changelog

## 0.7.0

مستند کامل نسخه: [docs/project/releases/v0.7.0.md](docs/project/releases/v0.7.0.md)

- Added the `float` type, decimal literals, arithmetic, comparisons and arrays.
- Added explicit `to_float` conversion and integral-float support in `to_int`.
- Added Unicode-aware text case, trimming, search, replace, split, join, repeat and slice helpers.
- Added numeric absolute value, min/max, clamp, power, square root and rounding helpers.
- Added non-mutating array reverse, sort, slice, sum, insert and remove helpers.
- Added negative indexing for slice/remove-style helpers and resource limits for repeated text.
- Added a dedicated v0.7 positive, negative and runtime-edge test suite.

## 0.6.0

مستند کامل نسخه: [docs/project/releases/v0.6.0.md](docs/project/releases/v0.6.0.md)

- Added `final` typed bindings with compile-time reassignment checks.
- Added Python-style `for name in iterable` loops for arrays and strings.
- Expanded `range` to one, two, or three arguments, including negative steps.
- Added `input()` and `input(prompt)` while retaining `read_line()` compatibility.
- Expanded `print` to multiple arguments plus `sep` and `end` named options.
- Allowed `if` and `while` conditions with or without parentheses.
- Added exact-text output capture and streaming stdio execution.
- Added a dedicated v0.6 behavior and error suite.

## 0.5.0

مستند کامل نسخه: [docs/project/releases/v0.5.0.md](docs/project/releases/v0.5.0.md)

- Added lexical block scopes and shadowing.
- Added `if`, `else`, `while`, and C-style `for`.
- Added `break`, `continue`, and `return` validation.
- Added typed functions, forward calls, and recursion.
- Added typed and multidimensional arrays, indexing, and mutation.
- Added `%`, compound assignments, increment, and decrement.
- Added `len`, `range`, `append`, `to_string`, and `to_int`.
- Added CLI check, eval, stdin, token, and AST modes.
- Added semantic return-path, arity, scope, and type checks.
- Expanded the automated suite while preserving v0.1 behavior.

## 0.1.0

مستند کامل نسخه: [docs/project/releases/v0.1.0.md](docs/project/releases/v0.1.0.md)

- Initial Rust rewrite with variables, expressions, strings, booleans, printing, static checks, and source-aware diagnostics.
