use persica::{compile, run};

fn run_ok(source: &str) -> Vec<String> {
    run(source).expect("source should execute without runtime errors")
}

fn check_error(source: &str, expected_message: &str) {
    let result = compile(source);
    assert!(result.is_err(), "expected compile error, got Ok");
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains(expected_message),
        "expected error containing '{expected_message}', but got: {err}"
    );
}

fn check_runtime_error(source: &str, expected_message: &str) {
    let result = run(source);
    assert!(result.is_err(), "expected runtime error, got Ok");
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains(expected_message),
        "expected runtime error containing '{expected_message}', but got: {err}"
    );
}

#[test]
fn test_basic_throw_and_catch_string() {
    let program = r#"
try {
    print("Before throw");
    throw "Something went wrong!";
    print("After throw - unreachable");
} catch (error) {
    print("Caught error:", error);
}
print("Execution continues smoothly");
"#;

    let output = run_ok(program);
    assert_eq!(
        output,
        vec![
            "Before throw",
            "Caught error: Something went wrong!",
            "Execution continues smoothly"
        ]
    );
}

#[test]
fn test_catch_syntax_variations() {
    // 1. catch without parentheses: catch error
    let p1 = r#"
try {
    throw "err1";
} catch error {
    print("v1:", error);
}
"#;
    assert_eq!(run_ok(p1), vec!["v1: err1"]);

    // 2. catch with explicit type: catch (string e)
    let p2 = r#"
try {
    throw "err2";
} catch (string e) {
    print("v2:", e);
}
"#;
    assert_eq!(run_ok(p2), vec!["v2: err2"]);

    // 3. anonymous catch block: catch { ... }
    let p3 = r#"
try {
    throw "err3";
} catch {
    print("v3: caught anonymously");
}
"#;
    assert_eq!(run_ok(p3), vec!["v3: caught anonymously"]);
}

#[test]
fn test_throw_and_catch_numbers_and_objects() {
    let program = r#"
// Throw int error code
try {
    throw 404;
} catch (code) {
    print("HTTP Code:", code);
}

// Throw dictionary error details
try {
    throw {"status": 500, "message": "Database connection lost"};
} catch (details) {
    print("Failed with status:", details["status"]);
    print("Details:", details["message"]);
}

// Throw array
try {
    throw [10, 20, 30];
} catch (arr) {
    print("Array length thrown:", len(arr));
}
"#;

    let output = run_ok(program);
    assert_eq!(
        output,
        vec![
            "HTTP Code: 404",
            "Failed with status: 500",
            "Details: Database connection lost",
            "Array length thrown: 3"
        ]
    );
}

#[test]
fn test_catch_runtime_division_by_zero() {
    let program = r#"
int count = 0;
try {
    int res = 100 / count;
    print("Result:", res);
} catch (err) {
    print("Caught division by zero:", err);
}
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["Caught division by zero: division by zero"]);
}

#[test]
fn test_catch_runtime_array_index_out_of_bounds() {
    let program = r#"
int[] numbers = [10, 20, 30];
try {
    print(numbers[99]);
} catch (err) {
    print("Handled bounds error successfully");
}
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["Handled bounds error successfully"]);
}

#[test]
fn test_catch_runtime_dict_missing_key() {
    let program = r#"
dict user = {"name": "Persica"};
try {
    print(user["missing_key"]);
} catch (err) {
    print("Key missing caught:", err);
}
"#;

    let output = run_ok(program);
    assert_eq!(
        output,
        vec!["Key missing caught: key missing_key not found in map"]
    );
}

#[test]
fn test_exception_unwinds_across_function_calls() {
    let program = r#"
def void level3() {
    print("level3 entered");
    throw "Error from deepest level!";
    print("level3 finished - unreachable");
}

def void level2() {
    print("level2 entered");
    level3();
    print("level2 finished - unreachable");
}

def void level1() {
    print("level1 entered");
    level2();
    print("level1 finished - unreachable");
}

try {
    level1();
} catch (e) {
    print("Caught in main:", e);
}
"#;

    let output = run_ok(program);
    assert_eq!(
        output,
        vec![
            "level1 entered",
            "level2 entered",
            "level3 entered",
            "Caught in main: Error from deepest level!"
        ]
    );
}

#[test]
fn test_nested_try_catch_and_rethrow() {
    let program = r#"
try {
    print("Outer try start");
    try {
        print("Inner try start");
        throw "Inner exception";
    } catch (string inner_err) {
        print("Inner catch caught:", inner_err);
        throw "Re-thrown: " + inner_err;
    }
} catch (string outer_err) {
    print("Outer catch caught:", outer_err);
}
"#;

    let output = run_ok(program);
    assert_eq!(
        output,
        vec![
            "Outer try start",
            "Inner try start",
            "Inner catch caught: Inner exception",
            "Outer catch caught: Re-thrown: Inner exception"
        ]
    );
}

#[test]
fn test_returns_from_within_try_and_catch() {
    let program = r#"
def int test_try_return(bool should_fail) {
    try {
        if (should_fail) {
            throw "Failed!";
        }
        return 100;
    } catch (err) {
        return -1;
    }
}

print(test_try_return(false));
print(test_try_return(true));
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["100", "-1"]);
}

#[test]
fn test_break_and_continue_inside_try_loop() {
    let program = r#"
int sum = 0;
for (int i = 0; i < 10; i++) {
    try {
        if (i == 3) {
            continue;
        }
        if (i == 7) {
            break;
        }
        sum += i;
    } catch (e) {
        print("Never reached");
    }
}
print("Sum:", sum);
"#;

    let output = run_ok(program);
    // 0 + 1 + 2 (skip 3) + 4 + 5 + 6 = 18
    assert_eq!(output, vec!["Sum: 18"]);
}

#[test]
fn test_throw_statement_satisfies_function_return_requirement() {
    let program = r#"
def int calculate(int x) {
    if (x > 0) {
        return x * 2;
    }
    throw "Negative x not allowed";
}

try {
    print(calculate(5));
    print(calculate(-3));
} catch (err) {
    print("Handled:", err);
}
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["10", "Handled: Negative x not allowed"]);
}

#[test]
fn test_cannot_throw_void_value_semantic_error() {
    check_error(
        r#"
def void no_value() {
}

throw no_value();
"#,
        "cannot throw a void value",
    );
}

#[test]
fn test_unhandled_throw_reports_runtime_diagnostic() {
    check_runtime_error(
        r#"
throw "Uncaught crisis!";
"#,
        "Uncaught crisis!",
    );
}
