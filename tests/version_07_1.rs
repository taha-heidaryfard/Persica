use persica::{compile, diagnostic::Phase, run, runtime::Interpreter};

#[test]
fn min_and_max_work_on_arrays_and_scalars() {
    assert_eq!(
        run(r#"
            int[] numbers = [42, 10, 85, 3, 19];
            print(min(numbers), max(numbers));

            float[] prices = [19.99, 5.50, 42.0, 3.25];
            print(min(prices), max(prices));

            string[] words = ["persica", "apple", "zebra", "banana"];
            print(min(words), max(words));

            string[] sample = ["persica", "apple", "rust"];
            print(min(sample), max(sample));

            print(min(100, 20), max(100, 20));
            print(min(1.5, 2.5), max(1.5, 2.5));
            "#)
        .unwrap(),
        [
            "3 85",
            "3.25 42.0",
            "apple persica",
            "rust persica",
            "20 100",
            "1.5 2.5"
        ]
    );
}

#[test]
fn mean_computes_arithmetic_average() {
    assert_eq!(
        run(r#"
            int[] scores = [10, 20, 30, 40];
            print(mean(scores));

            float[] values = [1.5, 2.5, 5.0];
            print(mean(values));
            "#)
        .unwrap(),
        ["25.0", "3.0"]
    );
}

#[test]
fn median_handles_odd_and_even_lengths() {
    assert_eq!(
        run(r#"
            int[] odd_int = [9, 1, 5];
            print(median(odd_int));

            int[] even_int = [10, 20, 30, 40];
            print(median(even_int));

            float[] odd_float = [5.5, 1.1, 3.3];
            print(median(odd_float));

            float[] even_float = [1.0, 2.0, 3.0, 4.0];
            print(median(even_float));
            "#)
        .unwrap(),
        ["5.0", "25.0", "3.3", "2.5"]
    );
}

#[test]
fn variance_and_stdev_calculate_population_statistics() {
    assert_eq!(
        run(r#"
            int[] data = [2, 4, 4, 4, 5, 5, 7, 9];
            print(variance(data));
            print(stdev(data));

            float[] float_data = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
            print(variance(float_data));
            print(stdev(float_data));
            "#)
        .unwrap(),
        ["4.0", "2.0", "4.0", "2.0"]
    );
}

#[test]
fn empty_arrays_report_controlled_runtime_errors() {
    for source in [
        "int[] values = []; print(min(values));",
        "float[] values = []; print(max(values));",
        "int[] values = []; print(mean(values));",
        "float[] values = []; print(median(values));",
        "int[] values = []; print(variance(values));",
        "float[] values = []; print(stdev(values));",
    ] {
        let compilation = compile(source).unwrap_or_else(|error| panic!("{source}: {error}"));
        let error = Interpreter::new()
            .interpret(&compilation.program)
            .unwrap_err();
        assert_eq!(error.phase, Phase::Runtime, "{source}");
        assert!(
            error.message.contains("cannot be called on an empty array"),
            "{source}"
        );
    }
}

#[test]
fn statistics_builtins_type_and_arity_checking() {
    for source in [
        "print(mean([\"a\", \"b\"]));",
        "print(median([true, false]));",
        "print(variance([\"text\"]));",
        "print(stdev([true]));",
        "print(mean());",
        "print(mean([1], [2]));",
        "print(min());",
        "print(min(1, 2, 3));",
    ] {
        let error = compile(source).unwrap_err();
        assert_eq!(error.phase, Phase::Semantic, "{source}");
    }
}

#[test]
fn reserved_builtin_names_cannot_be_redefined() {
    for source in [
        "def void mean() {}",
        "def void median() {}",
        "def void variance() {}",
        "def void stdev() {}",
    ] {
        let error = compile(source).unwrap_err();
        assert_eq!(error.phase, Phase::Semantic, "{source}");
        assert!(
            error.message.contains("reserved built-in function"),
            "{source}"
        );
    }
}
