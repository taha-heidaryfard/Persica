use std::{fs, process::Command};

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_persica"))
}

#[test]
fn test_persica_toml_explicit_with_entry_file() {
    // Tests: persica persica.toml src/main.persi
    let temp_dir = std::env::temp_dir().join(format!("persica_v13_expl_{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(temp_dir.join("src")).expect("create src");
    fs::create_dir_all(temp_dir.join(".persi_env")).expect("create .persi_env");

    let toml_content = r#"
[package]
name = "explicit_test"
version = "0.1.0"
entry = "src/main.persi"

[paths]
source = "src"
packages = ".persi_env"
"#;
    fs::write(temp_dir.join("persica.toml"), toml_content).expect("write toml");

    // Put a library inside .persi_env/
    let pkg_dir = temp_dir.join(".persi_env").join("my_math");
    fs::create_dir_all(&pkg_dir).expect("create my_math");
    fs::write(
        pkg_dir.join("mod.persi"),
        r#"
def int multiply(int a, int b) {
    return a * b;
}
"#,
    )
    .expect("write my_math/mod.persi");

    // Put main inside src/
    let main_code = r#"
from my_math import multiply;

print("Result:", multiply(6, 7));
"#;
    fs::write(temp_dir.join("src").join("main.persi"), main_code).expect("write main");

    // Execute: persica persica.toml src/main.persi
    let toml_path = temp_dir.join("persica.toml").to_string_lossy().to_string();
    let main_path = temp_dir
        .join("src")
        .join("main.persi")
        .to_string_lossy()
        .to_string();

    let output = binary()
        .args([&toml_path, &main_path])
        .output()
        .expect("run persica toml + main");

    let _ = fs::remove_dir_all(&temp_dir);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap().trim(),
        "Result: 42"
    );
}

#[test]
fn test_persica_toml_only_runs_entry() {
    // Tests: persica persica.toml (runs entry defined in toml)
    let temp_dir =
        std::env::temp_dir().join(format!("persica_v13_toml_only_{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(temp_dir.join("src")).expect("create src");
    fs::create_dir_all(temp_dir.join(".persi_env")).expect("create .persi_env");

    let toml_content = r#"
[package]
name = "auto_entry_app"
entry = "src/runner.persi"

[paths]
source = "src"
packages = ".persi_env"
"#;
    fs::write(temp_dir.join("persica.toml"), toml_content).expect("write toml");

    fs::write(
        temp_dir.join("src").join("runner.persi"),
        r#"
print("Executed via auto-entry!");
"#,
    )
    .expect("write runner.persi");

    let toml_path = temp_dir.join("persica.toml").to_string_lossy().to_string();

    let output = binary().arg(&toml_path).output().expect("run persica toml");

    let _ = fs::remove_dir_all(&temp_dir);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap().trim(),
        "Executed via auto-entry!"
    );
}

#[test]
fn test_cli_new_scaffolds_project_with_persi_env() {
    let temp_parent =
        std::env::temp_dir().join(format!("persica_cli_new_parent_{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp_parent);
    fs::create_dir_all(&temp_parent).expect("create parent");

    let project_name = "sample_app";
    let project_dir = temp_parent.join(project_name);

    let output = binary()
        .current_dir(&temp_parent)
        .args(["new", project_name])
        .output()
        .expect("persica new");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify folders and files exist
    assert!(project_dir.join("persica.toml").is_file());
    assert!(project_dir.join(".persi_env").is_dir());
    assert!(project_dir.join("src").join("main.persi").is_file());
    assert!(project_dir.join("tests").join("test_main.persi").is_file());

    // Verify project runs out of the box with `persica`
    let run_output = binary()
        .current_dir(&project_dir)
        .output()
        .expect("persica auto run");

    assert!(run_output.status.success());
    assert!(
        String::from_utf8(run_output.stdout)
            .unwrap()
            .contains("Hello from sample_app!")
    );

    let _ = fs::remove_dir_all(&temp_parent);
}

#[test]
fn test_cli_test_command_runs_tests() {
    let temp_dir = std::env::temp_dir().join(format!("persica_cli_test_{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(temp_dir.join("tests")).expect("create tests");

    fs::write(
        temp_dir.join("tests").join("test_first.persi"),
        r#"
print("Test 1 ran");
"#,
    )
    .expect("write test_first");

    fs::write(
        temp_dir.join("tests").join("test_second.persi"),
        r#"
print("Test 2 ran");
"#,
    )
    .expect("write test_second");

    let output = binary()
        .current_dir(&temp_dir)
        .arg("test")
        .output()
        .expect("run persica test");

    let _ = fs::remove_dir_all(&temp_dir);

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("running 2 test file(s)"));
    assert!(stdout.contains("test result: ok. 2 passed; 0 failed"));
}

#[test]
fn test_single_file_standalone_without_config() {
    let temp_dir = std::env::temp_dir().join(format!("persica_standalone_{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).expect("create temp");

    let file_path = temp_dir.join("simple.persi");
    fs::write(&file_path, "print(100 + 200);").expect("write simple.persi");

    let output = binary()
        .arg(file_path.to_string_lossy().to_string())
        .output()
        .expect("run standalone");

    let _ = fs::remove_dir_all(&temp_dir);

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap().trim(), "300");
}
