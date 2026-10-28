use persica::run;

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
fn struct_pure_data_model() {
    let source = r#"
        struct Point {
            int x;
            int y;
        }

        Point p = Point(10, 20);
        print(p.x);
        print(p.y);

        p.x = 42;
        print(p.x);
        print(p);
    "#;
    let output = run_ok(source);
    assert_eq!(output, vec!["10", "20", "42", "Point { x: 42, y: 20 }"]);
}

#[test]
fn struct_value_semantics_copy() {
    let source = r#"
        struct Point {
            int x;
            int y;
        }

        Point p1 = Point(1, 2);
        Point p2 = p1;
        p2.x = 99;

        print(p1.x);
        print(p2.x);
        print(p1 == p2);
    "#;
    let output = run_ok(source);
    assert_eq!(output, vec!["1", "99", "false"]);
}

#[test]
fn struct_equality_and_nesting() {
    let source = r#"
        struct Point {
            int x;
            int y;
        }

        Point a = Point(5, 10);
        Point b = Point(5, 10);
        Point c = Point(5, 11);

        print(a == b);
        print(a == c);
        print(a != c);

        struct Line {
            Point start;
            Point end;
        }

        Line line = Line(Point(0, 0), Point(10, 20));
        print(line.start.x);
        print(line.end.y);

        line.start.x = 100;
        print(line.start.x);
    "#;
    let output = run_ok(source);
    assert_eq!(output, vec!["true", "false", "true", "0", "20", "100"]);
}

#[test]
fn class_object_and_behavior() {
    let source = r#"
        class User {
            string name;
            int age;

            def introduce() {
                print(name);
            }

            def void have_birthday(int years) {
                this.age = this.age + years;
            }

            def int get_age() {
                return age;
            }
        }

        User u = User("طاها", 22);
        u.introduce();
        print(u.get_age());

        u.have_birthday(1);
        print(u.get_age());
        print(u.age);
    "#;
    let output = run_ok(source);
    assert_eq!(output, vec!["طاها", "22", "23", "23"]);
}

#[test]
fn class_reference_semantics() {
    let source = r#"
        class Account {
            string owner;
            int balance;

            def void deposit(int amount) {
                balance = balance + amount;
            }
        }

        Account acc1 = Account("Taha", 1000);
        Account acc2 = acc1;

        acc2.deposit(500);

        print(acc1.balance);
        print(acc2.balance);
    "#;
    let output = run_ok(source);
    assert_eq!(output, vec!["1500", "1500"]);
}

#[test]
fn class_methods_calling_methods() {
    let source = r#"
        class Counter {
            int count;

            def void inc() {
                count = count + 1;
            }

            def void inc_by(int n) {
                int i = 0;
                while i < n {
                    inc();
                    i = i + 1;
                }
            }

            def int value() {
                return count;
            }
        }

        Counter c = Counter(0);
        c.inc_by(5);
        print(c.value());
    "#;
    let output = run_ok(source);
    assert_eq!(output, vec!["5"]);
}

#[test]
fn structs_and_classes_in_collections() {
    let source = r#"
        struct Point {
            int x;
            int y;
        }

        class User {
            string name;
            def introduce() {
                print("Salam, " + name);
            }
        }

        Point[] points = [Point(1, 2), Point(3, 4)];
        print(points[0].x);
        print(points[1].y);
        points[0].x = 10;
        print(points[0].x);

        User[] users = [User("طاها"), User("علی")];
        users[0].introduce();
        users[1].introduce();

        map<string, Point> point_map = {
            "origin": Point(0, 0),
            "target": Point(100, 200)
        };
        print(point_map["target"].y);
    "#;
    let output = run_ok(source);
    assert_eq!(
        output,
        vec!["1", "4", "10", "Salam, طاها", "Salam, علی", "200"]
    );
}

#[test]
fn structs_reject_methods() {
    let source = r#"
        struct Point {
            int x;
            int y;

            def void move() {
                x = x + 1;
            }
        }
    "#;
    check_error(
        source,
        "structs cannot contain methods; use 'class' instead",
    );
}

#[test]
fn semantic_checks_and_type_safety() {
    // 1. Calling method on a struct
    let source1 = r#"
        struct Point {
            int x;
            int y;
        }
        Point p = Point(1, 2);
        p.test();
    "#;
    check_error(source1, "struct 'Point' has no methods");

    // 2. Accessing nonexistent field
    let source2 = r#"
        struct Point {
            int x;
            int y;
        }
        Point p = Point(1, 2);
        print(p.z);
    "#;
    check_error(source2, "struct 'Point' has no field 'z'");

    // 3. Type mismatch in field assignment
    let source3 = r#"
        struct Point {
            int x;
            int y;
        }
        Point p = Point(1, 2);
        p.x = "hello";
    "#;
    check_error(source3, "expected int, found string");

    // 4. Using 'this' outside class
    let source4 = r#"
        def void test() {
            print(this);
        }
    "#;
    check_error(source4, "'this' can only be used inside class methods");

    // 5. Wrong argument count in constructor
    let source5 = r#"
        struct Point {
            int x;
            int y;
        }
        Point p = Point(1);
    "#;
    check_error(source5, "function 'Point' expects 2 arguments, found 1");

    // 6. Duplicate field name
    let source6 = r#"
        struct Point {
            int x;
            int x;
        }
    "#;
    check_error(source6, "duplicate field 'x' in struct 'Point'");

    // 7. Duplicate method name
    let source7 = r#"
        class User {
            string name;
            def void foo() {}
            def void foo() {}
        }
    "#;
    check_error(source7, "duplicate method 'foo' in class 'User'");

    // 8. Member collision between field and method
    let source8 = r#"
        class User {
            string name;
            def void name() {}
        }
    "#;
    check_error(
        source8,
        "member 'name' is already defined as a field in class 'User'",
    );
}
