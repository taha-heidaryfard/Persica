use persica::{compile, diagnostic::Phase, run, runtime::Interpreter};

#[test]
fn supports_the_exact_typed_python_style_syntax() {
    let compilation = compile(
        r#"
        int x = to_int(input());
        for i in range(x) {
            print(i);
        }
        string name = "ali";
        print(name);
        "#,
    )
    .unwrap();

    let output = Interpreter::with_input(["3\n"])
        .interpret(&compilation.program)
        .unwrap();
    assert_eq!(output, ["0", "1", "2", "ali"]);
}

#[test]
fn input_prompt_and_python_style_print_preserve_exact_text() {
    let compilation = compile(
        r#"
        string name = input("Name: ");
        print("Hello", name, sep=", ", end="!");
        print(" Ready", end="");
        "#,
    )
    .unwrap();

    let output = Interpreter::with_input(["Ali\n"])
        .interpret_text(&compilation.program)
        .unwrap();
    assert_eq!(output, "Name: Hello, Ali! Ready");
}

#[test]
fn print_accepts_no_arguments_and_embedded_newlines() {
    let compilation = compile("print(); print(\"a\\nb\");").unwrap();
    let text = Interpreter::new()
        .interpret_text(&compilation.program)
        .unwrap();
    assert_eq!(text, "\na\nb\n");
    assert_eq!(run("print(); print(\"a\\nb\");").unwrap(), ["", "a\nb"]);
}

#[test]
fn final_variables_cannot_be_reassigned() {
    let error = compile("final int answer = 42; answer = 43;").unwrap_err();
    assert_eq!(error.phase, Phase::Semantic);
    assert!(error.message.contains("final"));
}

#[test]
fn final_arrays_keep_a_fixed_binding_but_allow_element_updates() {
    assert_eq!(
        run("final int[] values = [1, 2]; values[0] = 9; print(values);").unwrap(),
        ["[9, 2]"]
    );
    assert!(compile("final int[] values = [1]; values = [2];").is_err());
}

#[test]
fn range_supports_one_two_or_three_arguments() {
    assert_eq!(
        run(r#"
            print(range(4));
            print(range(2, 5));
            print(range(5, 0, -2));
            "#)
        .unwrap(),
        ["[0, 1, 2, 3]", "[2, 3, 4]", "[5, 3, 1]"]
    );

    let compilation = compile("print(range(0, 5, 0));").unwrap();
    let error = Interpreter::new()
        .interpret(&compilation.program)
        .unwrap_err();
    assert_eq!(error.phase, Phase::Runtime);
}

#[test]
fn for_in_iterates_arrays_and_unicode_strings() {
    assert_eq!(
        run(r#"
            for value in [2, 4] { print(value); }
            for letter in "آب" { print(letter); }
            "#)
        .unwrap(),
        ["2", "4", "آ", "ب"]
    );
}

#[test]
fn conditions_may_omit_parentheses() {
    assert_eq!(
        run(r#"
            int n = 0;
            while n < 2 { n++; }
            if n == 2 { print("ok"); }
            "#)
        .unwrap(),
        ["ok"]
    );
}

#[test]
fn print_named_arguments_are_checked() {
    assert!(compile("print(1, sep=2);").is_err());
    assert!(compile("print(1, unknown=\"x\");").is_err());
    assert!(compile("print(sep=\",\", 1);").is_err());
}
