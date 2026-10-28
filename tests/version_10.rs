use persica::{compile, register_virtual_module, run};

fn run_ok(source: &str) -> Vec<String> {
    run(source).expect("source should execute without runtime errors")
}

fn check_error(source: &str, expected_substring: &str) {
    let result = run(source);
    assert!(
        result.is_err(),
        "expected error containing '{expected_substring}', but execution succeeded"
    );
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains(expected_substring),
        "expected error to contain '{expected_substring}', got: {err}"
    );
}

#[test]
fn test_import_module_and_access_members() {
    register_virtual_module(
        "math_utils_10",
        r#"
final float PI = 3.14159;
def int add(int a, int b) {
    return a + b;
}
def float circle_area(float r) {
    return PI * r * r;
}
"#,
    );

    let program = r#"
import math_utils_10;

print(math_utils_10.add(10, 20));
print(math_utils_10.PI);
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["30", "3.14159"]);
}

#[test]
fn test_import_with_alias() {
    register_virtual_module(
        "calc_10",
        r#"
def int square(int x) {
    return x * x;
}
"#,
    );

    let program = r#"
import calc_10 as c;

print(c.square(6));
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["36"]);
}

#[test]
fn test_from_import_named_comma_separated() {
    register_virtual_module(
        "models_10",
        r#"
struct Point {
    int x;
    int y;
}

class User {
    string name;
    def greet() {
        print("Hello, " + this.name);
    }
}
"#,
    );

    let program = r#"
from models_10 import Point, User;

Point p = Point(10, 20);
print(p.x);

User u = User("Sara");
u.greet();
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["10", "Hello, Sara"]);
}

#[test]
fn test_from_import_with_parentheses_and_braces() {
    register_virtual_module(
        "shapes_10",
        r#"
struct Point {
    int x;
    int y;
}
struct Rect {
    int width;
    int height;
}
"#,
    );

    // Grouping with parentheses
    let prog1 = r#"
from shapes_10 import (Point, Rect);
Point p = Point(1, 2);
Rect r = Rect(10, 20);
print(p.x + r.width);
"#;
    assert_eq!(run_ok(prog1), vec!["11"]);

    // Grouping with curly braces
    let prog2 = r#"
from shapes_10 import {Point, Rect};
Point p = Point(5, 5);
Rect r = Rect(15, 25);
print(p.x + r.width);
"#;
    assert_eq!(run_ok(prog2), vec!["20"]);
}

#[test]
fn test_from_import_with_alias() {
    register_virtual_module(
        "geometry_10",
        r#"
struct Point {
    int x;
    int y;
}
def Point make_point(int x, int y) {
    return Point(x, y);
}
"#,
    );

    let program = r#"
from geometry_10 import Point as Pt, make_point as mk_pt;

Pt p = mk_pt(7, 9);
print(p.x);
print(p.y);
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["7", "9"]);
}

#[test]
fn test_from_import_star() {
    register_virtual_module(
        "toolkit_10",
        r#"
final int MAX_LIMIT = 100;
def int double_val(int n) {
    return n * 2;
}
struct Item {
    string title;
}
"#,
    );

    let program = r#"
from toolkit_10 import *;

print(MAX_LIMIT);
print(double_val(25));
Item it = Item("Book");
print(it.title);
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["100", "50", "Book"]);
}

#[test]
fn test_submodule_dot_notation() {
    register_virtual_module(
        "helpers.text.formatter",
        r#"
def string emphasize(string s) {
    return "*** " + s + " ***";
}
"#,
    );

    let program = r#"
from helpers.text.formatter import emphasize;

print(emphasize("Persica"));
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["*** Persica ***"]);
}

#[test]
fn test_submodule_import_as_namespace() {
    register_virtual_module(
        "helpers.math",
        r#"
def int mul(int a, int b) {
    return a * b;
}
"#,
    );

    let program = r#"
import helpers.math;

print(math.mul(4, 5));
"#;

    let output = run_ok(program);
    assert_eq!(output, vec!["20"]);
}

#[test]
fn test_import_symbol_not_found_reports_diagnostic() {
    register_virtual_module(
        "mod_a_10",
        r#"
def int existing_fn() {
    return 1;
}
"#,
    );

    let program = r#"
from mod_a_10 import non_existing_fn;
"#;

    let result = compile(program);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("symbol not found"),
        "expected symbol not found error, got: {err}"
    );
}

#[test]
fn test_circular_import_detection() {
    register_virtual_module(
        "circ_a_10",
        r#"
import circ_b_10;
def int from_a() { return 1; }
"#,
    );
    register_virtual_module(
        "circ_b_10",
        r#"
import circ_a_10;
def int from_b() { return 2; }
"#,
    );

    let program = r#"
import circ_a_10;
"#;

    let result = compile(program);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("circular import detected"),
        "expected circular import error, got: {err}"
    );
}

#[test]
fn test_type_checking_across_modules() {
    register_virtual_module(
        "typed_mod_10",
        r#"
def int take_int(int a) {
    return a;
}
"#,
    );

    check_error(
        r#"
from typed_mod_10 import take_int;
take_int("not an int");
"#,
        "expected int, found string",
    );
}

#[test]
fn test_disk_based_module_import() {
    let temp_dir = std::env::temp_dir().join(format!("persica_disk_mod_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&temp_dir);
    let mod_path = temp_dir.join("disk_math.persi");
    std::fs::write(
        &mod_path,
        r#"
def int cube(int x) {
    return x * x * x;
}
"#,
    )
    .expect("write disk module");

    let main_code = r#"
from disk_math import cube;
print(cube(3));
"#;

    let cur_dir = std::env::current_dir().unwrap();
    let _ = std::env::set_current_dir(&temp_dir);
    let res = run(main_code);
    let _ = std::env::set_current_dir(cur_dir);
    let _ = std::fs::remove_dir_all(&temp_dir);

    assert_eq!(res.expect("disk import succeeds"), vec!["27"]);
}
