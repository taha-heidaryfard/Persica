use persica::run;

fn run_ok(source: &str) -> Vec<String> {
    run(source).expect("source should execute without runtime errors")
}

#[test]
fn test_prelude_stack_auto_available() {
    let program = r#"
Stack s = Stack();
print(s.is_empty());
s.push("alpha");
s.push("beta");
s.push("gamma");

print(s.size());
print(s.peek());
print(s.pop());
print(s.pop());
print(s.size());
print(s.pop());
print(s.is_empty());
"#;

    let output = run_ok(program);
    assert_eq!(
        output,
        vec!["true", "3", "gamma", "gamma", "beta", "1", "alpha", "true"]
    );
}

#[test]
fn test_prelude_queue_auto_available() {
    let program = r#"
Queue q = Queue();
q.enqueue(100);
q.enqueue(200);
q.enqueue(300);

print(q.size());
print(q.peek());
print(q.dequeue());
print(q.dequeue());
print(q.dequeue());
print(q.is_empty());
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["3", "100", "100", "200", "300", "true"]);
}

#[test]
fn test_prelude_set_auto_available() {
    let program = r#"
Set set = Set();
set.add("apple");
set.add("banana");
set.add("apple");

print(set.size());
print(set.contains("apple"));
print(set.contains("orange"));

set.remove("apple");
print(set.size());
print(set.contains("apple"));
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["2", "true", "false", "1", "false"]);
}

#[test]
fn test_prelude_string_builder() {
    let program = r#"
StringBuilder sb = StringBuilder();
sb.append_str("Hello");
sb.append_str(" ");
sb.append_str("Persica");
sb.append_str("!");

print(sb.to_str());
print(sb.length());
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["Hello Persica!", "14"]);
}

#[test]
fn test_prelude_pair_and_result_and_option() {
    let program = r#"
Pair p = Pair(10, "persica");
print(p.first);
print(p.second);

Result res1 = ok(99);
print(res1.ok);
print(res1.value);

Result res2 = err("invalid input");
print(res2.ok);
print(res2.error);

Option opt1 = some(true);
print(opt1.has_value);
print(opt1.value);

Option opt2 = none();
print(opt2.has_value);
"#;

    let output = run_ok(program);
    assert_eq!(
        output,
        vec![
            "10",
            "persica",
            "true",
            "99",
            "false",
            "invalid input",
            "true",
            "true",
            "false"
        ]
    );
}

#[test]
fn test_prelude_algorithms_auto_available() {
    let program = r#"
any[][] pairs = zip([1, 2, 3], ["a", "b", "c"]);
print(len(pairs));
print(pairs[0][0]);
print(pairs[0][1]);
print(pairs[2][0]);
print(pairs[2][1]);

any[][] groups = chunks([10, 20, 30, 40, 50], 2);
print(len(groups));
print(groups[0]);
print(groups[2]);

any[] flat = flatten([[1, 2], [3, 4], [5]]);
print(flat);

int[] sorted = [10, 20, 30, 40, 50, 60, 70];
print(binary_search(sorted, 40));
print(binary_search(sorted, 99));
"#;

    let output = run_ok(program);
    assert_eq!(
        output,
        vec![
            "3",
            "1",
            "a",
            "3",
            "c",
            "3",
            "[10, 20]",
            "[50]",
            "[1, 2, 3, 4, 5]",
            "3",
            "-1"
        ]
    );
}

#[test]
fn test_stdlib_collections_module_import() {
    let program = r#"
from std.collections import Stack;

Stack s = Stack();
s.push(42);
print(s.pop());
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["42"]);
}

#[test]
fn test_stdlib_algorithms_module_import() {
    let program = r#"
from std.algorithms import binary_search, fill;

int[] arr = [5, 10, 15, 20, 25];
print(binary_search(arr, 15));

any[] filled = fill(4, "persi");
print(filled);
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["2", r#"["persi", "persi", "persi", "persi"]"#]);
}

#[test]
fn test_stdlib_strings_module_import() {
    let program = r#"
from std.strings import repeat_str, center, truncate;

print(repeat_str("abc", 3));
print(center("Title", 11, "-"));
print(truncate("This is a very long text", 10, "..."));
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["abcabcabc", "---Title---", "This is..."]);
}

#[test]
fn test_stdlib_math_module_import() {
    let program = r#"
from std.math import lerp, distance_2d;

print(lerp(0.0, 100.0, 0.5));
print(distance_2d(0.0, 0.0, 3.0, 4.0));
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["50.0", "5.0"]);
}

#[test]
fn test_user_function_can_shadow_prelude_function() {
    let program = r#"
def int binary_search(int x) {
    return x * 10;
}

print(binary_search(5));
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["50"]);
}

#[test]
fn test_user_struct_can_shadow_prelude_struct() {
    let program = r#"
struct Pair {
    int x;
    int y;
    int z;
}

Pair p = Pair(1, 2, 3);
print(p.x + p.y + p.z);
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["6"]);
}

#[test]
fn test_auto_stdlib_functions_work_without_any_import() {
    let program = r#"
// Testing functions directly without any import statement:
print(linear_search(["alpha", "beta", "gamma"], "beta"));
print(fill(3, 7));
print(repeat_str("Ha", 3));
print(center("TEST", 8, "*"));
print(truncate("LongStoryShort", 7, ".."));
print(lerp(10.0, 30.0, 0.5));
print(distance_2d(0.0, 0.0, 6.0, 8.0));
"#;

    let output = run_ok(program);
    assert_eq!(
        output,
        vec![
            "1",
            "[7, 7, 7]",
            "HaHaHa",
            "**TEST**",
            "LongS..",
            "20.0",
            "10.0"
        ]
    );
}
