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
fn python_style_dict_works_with_mixed_keys_and_values() {
    let source = r#"
        dict user = {
            "name": "طاها",
            "age": 22,
            1: "رتبه اول",
            true: "فعال"
        };
        print(user["name"]);
        print(user["age"]);
        print(user[1]);
        print(user[true]);
        print(len(user));
    "#;
    let output = run(source).unwrap();
    assert_eq!(output, ["طاها", "22", "رتبه اول", "فعال", "4"]);
}

#[test]
fn typed_map_enforces_key_and_value_types() {
    let source = r#"
        map<string, int> scores = {
            "ali": 20,
            "reza": 18
        };
        scores["sara"] = 19;
        print(scores["ali"]);
        print(scores["reza"]);
        print(scores["sara"]);
        print(len(scores));
    "#;
    let output = run(source).unwrap();
    assert_eq!(output, ["20", "18", "19", "3"]);
}

#[test]
fn map_any_any_explicit_syntax() {
    let source = r#"
        map<any, any> data = {
            1: "one",
            "two": 2
        };
        data[3] = "three";
        data["four"] = 4;
        print(data[1]);
        print(data["two"]);
        print(data[3]);
        print(data["four"]);
        print(len(data));
    "#;
    let output = run(source).unwrap();
    assert_eq!(output, ["one", "2", "three", "4", "4"]);
}

#[test]
fn map_string_any_for_json_like_data() {
    let source = r#"
        map<string, any> profile = {
            "username": "taha",
            "score": 100,
            "is_vip": true
        };
        profile["level"] = 5;
        print(profile["username"]);
        print(profile["score"]);
        print(profile["is_vip"]);
        print(profile["level"]);
    "#;
    let output = run(source).unwrap();
    assert_eq!(output, ["taha", "100", "true", "5"]);
}

#[test]
fn map_builtins_contains_keys_values_and_remove() {
    let source = r#"
        dict data = {
            "a": 1,
            "b": 2
        };
        print(contains_key(data, "a"));
        print(contains_key(data, "z"));

        string[] k = keys(data);
        print(len(k));
        print(k[0]);
        print(k[1]);

        int[] v = values(data);
        print(len(v));
        print(v[0]);
        print(v[1]);

        dict reduced = remove_key(data, "a");
        print(len(reduced));
        print(contains_key(reduced, "a"));
        print(reduced["b"]);
    "#;
    let output = run(source).unwrap();
    assert_eq!(
        output,
        [
            "true", "false", "2", "a", "b", "2", "1", "2", "1", "false", "2"
        ]
    );
}

#[test]
fn for_in_iterates_over_map_keys() {
    let source = r#"
        dict user = {
            "first": "طاها",
            "last": "رضایی"
        };
        for (k in user) {
            print(k, user[k]);
        }
    "#;
    let output = run(source).unwrap();
    assert_eq!(output, ["first طاها", "last رضایی"]);
}

#[test]
fn empty_dict_and_empty_map_literals() {
    let source = r#"
        dict empty_d = {};
        print(len(empty_d));
        empty_d["key"] = "val";
        print(len(empty_d));
        print(empty_d["key"]);

        map<string, int> empty_m = {};
        print(len(empty_m));
        empty_m["num"] = 42;
        print(empty_m["num"]);
    "#;
    let output = run(source).unwrap();
    assert_eq!(output, ["0", "1", "val", "0", "42"]);
}

#[test]
fn final_map_allows_entry_mutation_but_not_reassignment() {
    let source = r#"
        final dict config = {"theme": "dark"};
        config["theme"] = "light";
        config["lang"] = "fa";
        print(config["theme"]);
        print(config["lang"]);
    "#;
    let output = run(source).unwrap();
    assert_eq!(output, ["light", "fa"]);

    expect_semantic_error(
        r#"
        final dict config = {"theme": "dark"};
        config = {"theme": "light"};
        "#,
        "cannot assign to final variable 'config'",
    );
}

#[test]
fn map_runtime_domain_errors_do_not_panic() {
    expect_runtime_error(
        r#"
        dict m = {"a": 1};
        print(m["non_existent"]);
        "#,
        "key non_existent not found in map",
    );
}

#[test]
fn map_semantic_type_checking() {
    expect_semantic_error(
        r#"
        map<string, int> scores = {"ali": "twenty"};
        "#,
        "expected int, found string",
    );
    expect_semantic_error(
        r#"
        map<string, int> scores = {1: 20};
        "#,
        "expected string, found int",
    );
    expect_semantic_error(
        r#"
        map<string, int> scores = {"ali": 20};
        scores["ali"] = "error";
        "#,
        "expected int, found string",
    );
    expect_semantic_error(
        r#"
        map<string, int> scores = {"ali": 20};
        print(scores[1]);
        "#,
        "expected string, found int",
    );
}
