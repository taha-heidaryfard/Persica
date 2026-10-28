use persica::{compile, diagnostic::Phase, run};

fn expect_runtime_error(source: &str, expected_substring: &str) {
    match run(source) {
        Ok(_) => panic!(
            "expected runtime error containing '{expected_substring}', but execution succeeded"
        ),
        Err(diag) => {
            assert_eq!(diag.phase, Phase::Runtime);
            assert!(
                diag.message.contains(expected_substring),
                "expected error message to contain '{expected_substring}', got: '{}'",
                diag.message
            );
        }
    }
}

fn expect_semantic_error(source: &str, expected_substring: &str) {
    match compile(source) {
        Ok(_) => panic!(
            "expected semantic error containing '{expected_substring}', but validation succeeded"
        ),
        Err(diag) => {
            assert_eq!(diag.phase, Phase::Semantic);
            assert!(
                diag.message.contains(expected_substring),
                "expected error message to contain '{expected_substring}', got: '{}'",
                diag.message
            );
        }
    }
}

#[test]
fn registry_contains_over_100_functions() {
    let names = persica::builtins::all_builtin_names();
    assert!(names.len() >= 100);
    assert_eq!(names.len(), 115);
}

#[test]
fn math_constants_and_trig_functions() {
    let source = r#"
        float p = pi();
        float e_val = e();
        float t = tau();
        print(round(p * 100.0));
        print(round(e_val * 100.0));
        print(round(t * 100.0));

        float rad = deg_to_rad(180.0);
        print(round(rad * 100.0));
        float deg = rad_to_deg(p);
        print(round(deg));

        print(round(sin(deg_to_rad(90.0))));
        print(round(cos(0.0)));
        print(round(tan(deg_to_rad(45.0))));

        print(round(rad_to_deg(asin(1.0))));
        print(round(rad_to_deg(acos(1.0))));
        print(round(rad_to_deg(atan(1.0))));
        print(round(rad_to_deg(atan2(1.0, 1.0))));
    "#;
    let output = run(source).unwrap();
    assert_eq!(
        output,
        [
            "314", "272", "628", "314", "180", "1", "1", "1", "90", "0", "45", "45"
        ]
    );
}

#[test]
fn math_hyperbolic_logs_and_powers() {
    let source = r#"
        print(round(sinh(0.0)));
        print(round(cosh(0.0)));
        print(round(tanh(0.0)));

        print(round(hypot(3.0, 4.0)));
        print(round(cbrt(27.0)));

        print(round(ln(e())));
        print(round(log2(8.0)));
        print(round(log10(1000.0)));

        print(round(exp(0.0)));
        print(round(exp2(3.0)));

        print(round(fract(3.75) * 100.0));
        print(round(trunc(3.75)));
    "#;
    let output = run(source).unwrap();
    assert_eq!(
        output,
        ["0", "1", "0", "5", "3", "1", "3", "3", "1", "8", "75", "3"]
    );
}

#[test]
fn math_number_theory_and_bases() {
    let source = r#"
        print(gcd(48, 18));
        print(lcm(12, 15));
        print(factorial(5));
        print(factorial(0));
        print(is_prime(17));
        print(is_prime(18));
        print(next_prime(14));

        print(sign(-42));
        print(sign(0));
        print(sign(42));
        print(round(sign_float(-3.14)));
        print(round(copysign(5.0, -1.0)));

        print(is_even(4));
        print(is_odd(5));
        print(is_between(5, 1, 10));

        print(to_bin(10));
        print(from_bin("1010"));
        print(to_hex(255));
        print(from_hex("ff"));
    "#;
    let output = run(source).unwrap();
    assert_eq!(
        output,
        [
            "6", "60", "120", "1", "true", "false", "17", "-1", "0", "1", "-1", "-5", "true",
            "true", "true", "1010", "10", "ff", "255"
        ]
    );
}

#[test]
fn math_float_inspection() {
    let source = r#"
        print(is_finite(3.14));
        print(is_infinite(3.14));
        print(is_nan(3.14));
    "#;
    let output = run(source).unwrap();
    assert_eq!(output, ["true", "false", "false"]);
}

#[test]
fn string_utilities_transform_and_align() {
    let source = r#"
        print(capitalize("persica"));
        print(title_case("hello world persica"));

        print(pad_left("42", 5, "0"));
        print(pad_right("hi", 4, "."));
        print(center("rust", 8, "-"));

        print(char_at("Persica", 0));
        print(char_at("Persica", 6));
        print(code_point("A"));
        print(from_code_point(65));

        print(remove_prefix("foobar", "foo"));
        print(remove_suffix("foobar", "bar"));
        print(common_prefix("persica_lang", "persica_tool"));
        print(common_suffix("test.rs", "main.rs"));

        string[] compacted = compact_string(["a", "", "b", ""]);
        print(len(compacted));
        print(compacted[0]);
        print(compacted[1]);
    "#;
    let output = run(source).unwrap();
    assert_eq!(
        output,
        [
            "Persica",
            "Hello World Persica",
            "00042",
            "hi..",
            "--rust--",
            "P",
            "a",
            "65",
            "A",
            "bar",
            "foo",
            "persica_",
            ".rs",
            "2",
            "a",
            "b"
        ]
    );
}

#[test]
fn string_search_and_analysis() {
    let source = r#"
        print(index_of_char("apple", "p"));
        print(index_of_char("apple", "z"));
        print(count_char("banana", "a"));

        print(is_alphabetic("Persica"));
        print(is_alphabetic("Persica123"));
        print(is_numeric("12345"));
        print(is_alphanumeric("Persica42"));
        print(is_whitespace("   \t\n "));
        print(is_upper("HELLO"));
        print(is_lower("hello"));

        print(levenshtein("kitten", "sitting"));
    "#;
    let output = run(source).unwrap();
    assert_eq!(
        output,
        [
            "1", "-1", "3", "true", "false", "true", "true", "true", "true", "true", "3"
        ]
    );
}

#[test]
fn string_splits_into_arrays() {
    let source = r#"
        string[] c = chars("سلام");
        print(len(c));
        print(c[0]);
        print(c[3]);

        string[] w = words("سلام دنیا پرسیکا");
        print(len(w));
        print(w[1]);

        string[] l = lines("line1\nline2\r\nline3");
        print(len(l));
        print(l[2]);
    "#;
    let output = run(source).unwrap();
    assert_eq!(output, ["4", "س", "م", "3", "دنیا", "3", "line3"]);
}

#[test]
fn array_selection_and_slicing() {
    let source = r#"
        int[] nums = [10, 20, 30, 40, 50];
        print(first(nums));
        print(last(nums));

        int[] t = take(nums, 3);
        print(len(t));
        print(t[2]);

        int[] d = drop(nums, 3);
        print(len(d));
        print(d[0]);

        string[] s = ["a", "b", "c"];
        print(first(s));
        print(last(s));
    "#;
    let output = run(source).unwrap();
    assert_eq!(output, ["10", "50", "3", "30", "2", "40", "a", "c"]);
}

#[test]
fn array_unique_flatten_chunk_fill() {
    let source = r#"
        int[] u = unique_int([1, 2, 2, 3, 1, 4, 3]);
        print(len(u));
        print(u[0]);
        print(u[1]);
        print(u[2]);
        print(u[3]);

        int[][] nested = [[1, 2], [3, 4, 5], [6]];
        int[] flat = flatten_int(nested);
        print(len(flat));
        print(flat[5]);

        int[][] chunks = chunk_int([1, 2, 3, 4, 5], 2);
        print(len(chunks));
        print(len(chunks[0]));
        print(len(chunks[2]));

        int[] filled = fill_int(3, 99);
        print(len(filled));
        print(filled[0]);
        print(filled[2]);

        string[] fstr = fill_string(2, "ok");
        print(fstr[0]);
        print(fstr[1]);
    "#;
    let output = run(source).unwrap();
    assert_eq!(
        output,
        [
            "4", "1", "2", "3", "4", "6", "6", "3", "2", "1", "3", "99", "99", "ok", "ok"
        ]
    );
}

#[test]
fn array_search_and_rotation() {
    let source = r#"
        int[] nums = [10, 20, 30, 20, 10];
        print(index_of_int(nums, 20));
        print(last_index_of_int(nums, 20));
        print(count_int(nums, 20));

        int[] merged = concat_int([1, 2], [3, 4]);
        print(len(merged));
        print(merged[3]);

        int[] swapped = swap_int([10, 20, 30], 0, 2);
        print(swapped[0]);
        print(swapped[2]);

        print(is_empty(""));
        print(is_empty("persica"));
        int[] head_nums = head(nums);
        print(len(head_nums));
        print(head_nums[0]);
        int[] rest = tail(nums);
        print(len(rest));
        print(rest[0]);
    "#;
    let output = run(source).unwrap();
    assert_eq!(
        output,
        [
            "1", "3", "2", "4", "4", "30", "10", "true", "false", "4", "10", "4", "20"
        ]
    );
}

#[test]
fn stats_products_means_and_geometry() {
    let source = r#"
        print(product_int([1, 2, 3, 4]));
        print(round(product_float([1.5, 2.0])));

        float gm = geometric_mean([2.0, 8.0]);
        print(round(gm));

        float hm = harmonic_mean([2.0, 6.0]);
        print(round(hm * 10.0));

        print(mode_int([1, 2, 2, 3]));

        print(round(lerp(10.0, 20.0, 0.5)));

        float[] norm = normalize([10.0, 20.0]);
        print(round(norm[0]));
        print(round(norm[1]));

        print(round(distance_2d(0.0, 0.0, 3.0, 4.0)));
        print(round(distance_3d(0.0, 0.0, 0.0, 1.0, 2.0, 2.0)));

        print(round(z_score(15.0, 10.0, 2.5)));
    "#;
    let output = run(source).unwrap();
    assert_eq!(
        output,
        ["24", "3", "4", "30", "2", "15", "0", "1", "5", "3", "2"]
    );
}

#[test]
fn registry_domain_errors_do_not_panic() {
    expect_runtime_error(
        "print(asin(2.0));",
        "asin argument must be between -1.0 and 1.0",
    );
    expect_runtime_error(
        "print(acos(-2.0));",
        "acos argument must be between -1.0 and 1.0",
    );
    expect_runtime_error("print(ln(-5.0));", "ln expects positive argument");
    expect_runtime_error("print(log2(-8.0));", "log2 expects positive argument");
    expect_runtime_error(
        "print(factorial(-1));",
        "factorial expects an integer between 0 and 20",
    );
    expect_runtime_error(
        "print(factorial(25));",
        "factorial expects an integer between 0 and 20",
    );
    expect_runtime_error(
        "int[] a = []; print(first(a));",
        "first cannot be called on an empty array",
    );
    expect_runtime_error(
        "int[] a = []; print(last(a));",
        "last cannot be called on an empty array",
    );
    expect_runtime_error(
        "print(chunk_int([1, 2], 0));",
        "chunk size must be greater than zero",
    );
    expect_runtime_error(
        "print(fill_int(-1, 5));",
        "fill_int count must be between 0 and 1,000,000",
    );
    expect_runtime_error("print(char_at(\"abc\", 5));", "out of bounds");
    expect_runtime_error(
        "float[] empty = []; print(geometric_mean(empty));",
        "geometric_mean cannot be called on an empty array",
    );
}

#[test]
fn registry_type_and_arity_checking() {
    expect_semantic_error("print(sin());", "sin expects 1 argument, found 0");
    expect_semantic_error("print(sin(1.0, 2.0));", "sin expects 1 argument, found 2");
    expect_semantic_error("print(sin(42));", "expected float, found int");
    expect_semantic_error(
        "print(pad_left(42, 5, \"0\"));",
        "expected string, found int",
    );
    expect_semantic_error(
        "print(chunk_int([\"a\"], 2));",
        "expected int[], found string[]",
    );
}

#[test]
fn user_function_can_shadow_registry_builtin() {
    let source = r#"
        def int gcd(int a, int b) {
            return 999;
        }
        print(gcd(10, 20));
    "#;
    let output = run(source).unwrap();
    assert_eq!(output, ["999"]);
}
