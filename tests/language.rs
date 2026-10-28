use persica::{compile, diagnostic::Phase, run};

#[test]
fn runs_the_v01_language_surface() {
    let source = r#"
        int a = 10;
        int b = 20;
        bool ok = a < b && true;
        string name = "Persica";

        print(a + b * 2);
        print(ok);
        print("Hello " + name);
        a = 30;
        print(a);
    "#;

    assert_eq!(run(source).unwrap(), ["50", "true", "Hello Persica", "30"]);
}

#[test]
fn assignment_is_right_associative() {
    let source = "int a = 1; int b = 2; a = b = 9; print(a); print(b);";
    assert_eq!(run(source).unwrap(), ["9", "9"]);
}

#[test]
fn logical_operators_short_circuit() {
    let source = "print(false && (1 / 0 == 0)); print(true || (1 / 0 == 0));";
    assert_eq!(run(source).unwrap(), ["false", "true"]);
}

#[test]
fn supports_unicode_names_comments_and_escapes() {
    let source = r#"
        /* Nested comments are supported: /* inner */ */
        int سن = 20;
        string پیام = "سال\nنو";
        // Print the values.
        print(سن);
        print(پیام);
    "#;
    assert_eq!(run(source).unwrap(), ["20", "سال\nنو"]);
}

#[test]
fn reports_static_type_mismatches() {
    let error = compile("int age = true;").unwrap_err();
    assert_eq!(error.phase, Phase::Semantic);
    assert!(error.message.contains("expected int, found bool"));
}

#[test]
fn reports_undefined_variables() {
    let error = compile("print(missing);").unwrap_err();
    assert_eq!(error.phase, Phase::Semantic);
    assert!(error.message.contains("undefined variable 'missing'"));
}

#[test]
fn rejects_duplicate_variables() {
    let error = compile("int value = 1; int value = 2;").unwrap_err();
    assert_eq!(error.phase, Phase::Semantic);
    assert!(error.message.contains("already defined"));
}

#[test]
fn rejects_unterminated_strings() {
    let error = compile("print(\"hello);").unwrap_err();
    assert_eq!(error.phase, Phase::Lexer);
    assert!(error.message.contains("unterminated string"));
}

#[test]
fn reports_parser_errors_with_a_span() {
    let error = compile("int value = 10").unwrap_err();
    assert_eq!(error.phase, Phase::Parser);
    assert!(error.message.contains("expected ';'"));
    assert!(error.span.is_some());
}

#[test]
fn reports_division_by_zero_at_runtime() {
    let error = run("print(10 / 0);").unwrap_err();
    assert_eq!(error.phase, Phase::Runtime);
    assert_eq!(error.message, "division by zero");
}

#[test]
fn rejects_integer_literals_that_do_not_fit() {
    let error = compile("int huge = 999999999999999999999999;").unwrap_err();
    assert_eq!(error.phase, Phase::Parser);
    assert!(error.message.contains("out of range"));
}
