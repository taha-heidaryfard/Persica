use persica::{compile, diagnostic::Phase, run, runtime::Interpreter};

#[test]
fn blocks_shadow_without_overwriting_outer_variables() {
    let source = r#"
        int value = 1;
        {
            int value = 2;
            print(value);
        }
        print(value);
    "#;
    assert_eq!(run(source).unwrap(), ["2", "1"]);
}

#[test]
fn if_else_selects_one_branch() {
    let source = r#"
        int age = 20;
        if (age >= 18) { print("adult"); } else { print("minor"); }
    "#;
    assert_eq!(run(source).unwrap(), ["adult"]);
}

#[test]
fn while_break_continue_and_compound_assignment_work() {
    let source = r#"
        int i = 0;
        int sum = 0;
        while (i < 10) {
            i++;
            if (i % 2 == 0) { continue; }
            if (i > 7) { break; }
            sum += i;
        }
        print(sum);
    "#;
    assert_eq!(run(source).unwrap(), ["16"]);
}

#[test]
fn for_loop_keeps_its_initializer_scoped() {
    let source = r#"
        int sum = 0;
        for (int i = 0; i < 5; i++) { sum += i; }
        print(sum);
    "#;
    assert_eq!(run(source).unwrap(), ["10"]);
    let error = compile("for (int i = 0; i < 1; i++) {} print(i);").unwrap_err();
    assert!(error.message.contains("undefined variable 'i'"));
}

#[test]
fn typed_functions_support_recursion_and_forward_calls() {
    let source = r#"
        print(factorial(6));

        def int factorial(int n) {
            if (n <= 1) { return 1; }
            return n * factorial(n - 1);
        }
    "#;
    assert_eq!(run(source).unwrap(), ["720"]);
}

#[test]
fn void_functions_can_print_and_return_early() {
    let source = r#"
        def void greet(bool enabled) {
            if (!enabled) { return; }
            print("hello");
        }
        greet(false);
        greet(true);
    "#;
    assert_eq!(run(source).unwrap(), ["hello"]);
}

#[test]
fn arrays_support_indexing_mutation_and_nesting() {
    let source = r#"
        int[] values = [1, 2, 3];
        values[1] = 9;
        values[2] += 7;
        print(values);

        int[][] matrix = [[1, 2], [3, 4]];
        matrix[1][0] = 8;
        print(matrix[1][0]);
    "#;
    assert_eq!(run(source).unwrap(), ["[1, 9, 10]", "8"]);
}

#[test]
fn array_and_conversion_builtins_are_useful() {
    let source = r#"
        int[] empty = [];
        int[] values = range(2, 5);
        values = append(values, 9);
        print(len(empty));
        print(len(values));
        print(values);
        print(to_int("42") + 8);
        print("answer=" + to_string(50));
        print("Persica"[0]);
    "#;
    assert_eq!(
        run(source).unwrap(),
        ["0", "4", "[2, 3, 4, 9]", "50", "answer=50", "P"]
    );
}

#[test]
fn rejects_control_flow_in_the_wrong_context() {
    let break_error = compile("break;").unwrap_err();
    assert_eq!(break_error.phase, Phase::Semantic);
    let return_error = compile("return 1;").unwrap_err();
    assert_eq!(return_error.phase, Phase::Semantic);
}

#[test]
fn rejects_missing_returns_and_bad_calls() {
    let missing = compile("def int choose(bool ok) { if (ok) { return 1; } }").unwrap_err();
    assert!(missing.message.contains("every path"));

    let bad_call = compile("def int id(int n) { return n; } print(id(true));").unwrap_err();
    assert!(bad_call.message.contains("expected int, found bool"));
}

#[test]
fn reports_array_bounds_and_conversion_errors() {
    let bounds = run("int[] values = [1]; print(values[2]);").unwrap_err();
    assert_eq!(bounds.phase, Phase::Runtime);
    assert!(bounds.message.contains("out of bounds"));

    let conversion = run("print(to_int(\"not-a-number\"));").unwrap_err();
    assert_eq!(conversion.phase, Phase::Runtime);
    assert!(conversion.message.contains("not a valid int"));
}

#[test]
fn injected_input_makes_read_line_testable() {
    let compilation = compile("string name = read_line(); print(\"Hello \" + name);").unwrap();
    let output = Interpreter::with_input(["Taha\n"])
        .interpret(&compilation.program)
        .unwrap();
    assert_eq!(output, ["Hello Taha"]);
}
