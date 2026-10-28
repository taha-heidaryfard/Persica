use persica::{compile, diagnostic::Phase, run, runtime::Interpreter};

#[test]
fn float_literals_types_and_operators_work() {
    assert_eq!(
        run(r#"
            float price = 12.5;
            float tax = 2.5;
            print(price + tax, price - tax, price * 2.0, price / 2.0);
            print(-tax, price > tax, 1.5 == 1.5);
            "#)
        .unwrap(),
        ["15.0 10.0 25.0 6.25", "-2.5 true true"]
    );
}

#[test]
fn numeric_types_require_explicit_conversion() {
    let error = compile("float value = 1 + 2.0;").unwrap_err();
    assert_eq!(error.phase, Phase::Semantic);
    assert!(error.message.contains("matching int or float"));

    assert_eq!(
        run("float value = to_float(2) + 0.5; print(value); print(to_int(3.0));").unwrap(),
        ["2.5", "3"]
    );
}

#[test]
fn text_case_whitespace_and_matching_helpers_work() {
    assert_eq!(
        run(
            r#"
            string text = "  Persica Language  ";
            print(upper(trim(text)));
            print(lower("ABC"));
            print(trim_start("  a "), trim_end(" a  "), sep="|");
            print(contains(text, "Persica"), starts_with(trim(text), "Per"), ends_with(trim(text), "age"));
            "#
        )
        .unwrap(),
        ["PERSICA LANGUAGE", "abc", "a | a", "true true true"]
    );
}

#[test]
fn text_split_join_replace_repeat_find_count_and_slice_work() {
    assert_eq!(
        run(r#"
            string[] words = split("one,two,three", ",");
            print(join(" | ", words));
            print(replace("banana", "na", "NA"));
            print(repeat("ab", 3));
            print(find("سلام دنیا", "دنیا"), count("banana", "an"));
            print(slice("سلام", 1, -1));
            "#)
        .unwrap(),
        ["one | two | three", "baNANA", "ababab", "5 2", "لا"]
    );
}

#[test]
fn numeric_helpers_cover_int_and_float() {
    assert_eq!(
        run(r#"
            print(abs(-7), min(4, 9), max(4, 9), clamp(20, 0, 10));
            print(abs(-2.5), min(1.5, 2.5), max(1.5, 2.5), clamp(-1.0, 0.0, 5.0));
            print(pow(2.0, 3.0), sqrt(9.0));
            print(round(2.6), floor(2.9), ceil(2.1));
            "#)
        .unwrap(),
        ["7 4 9 10", "2.5 1.5 2.5 0.0", "8.0 3.0", "3 2 3"]
    );
}

#[test]
fn array_helpers_return_new_arrays() {
    assert_eq!(
        run(r#"
            int[] values = [3, 1, 2];
            print(sort(values));
            print(reverse(values));
            print(slice(values, 1));
            print(insert(values, 1, 9));
            print(remove(values, -1));
            print(sum(values), contains(values, 1));
            print(values);
            "#)
        .unwrap(),
        [
            "[1, 2, 3]",
            "[2, 1, 3]",
            "[1, 2]",
            "[3, 9, 1, 2]",
            "[3, 1]",
            "6 true",
            "[3, 1, 2]"
        ]
    );
}

#[test]
fn float_arrays_can_be_sorted_and_summed() {
    assert_eq!(
        run("float[] values = [2.5, 1.0, 3.5]; print(sort(values)); print(sum(values));").unwrap(),
        ["[1.0, 2.5, 3.5]", "7.0"]
    );
}

#[test]
fn unicode_text_operations_use_character_indices() {
    assert_eq!(
        run("print(len(\"ایران\"), reverse(\"ایران\"), slice(\"ایران\", -3));").unwrap(),
        ["5 ناریا ران"]
    );
}

#[test]
fn invalid_value_dependent_operations_report_runtime_errors() {
    for source in [
        "print(split(\"abc\", \"\"));",
        "print(sqrt(-1.0));",
        "print(clamp(1, 5, 2));",
        "int[] values = []; print(remove(values, 0));",
        "int[] values = []; print(sum(values));",
        "print(to_int(3.5));",
    ] {
        let compilation = compile(source).unwrap_or_else(|error| panic!("{source}: {error}"));
        let error = Interpreter::new()
            .interpret(&compilation.program)
            .unwrap_err();
        assert_eq!(error.phase, Phase::Runtime, "{source}");
    }
}

#[test]
fn builtin_types_and_arities_are_checked_statically() {
    for source in [
        "print(upper(1));",
        "print(join(\",\", [1, 2]));",
        "print(pow(2, 3));",
        "print(sort([true, false]));",
        "print(insert([1], 0, \"x\"));",
        "print(slice(1, 0));",
    ] {
        assert!(compile(source).is_err(), "{source}");
    }
}
