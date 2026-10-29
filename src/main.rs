use persica::{
    ProjectConfig, clear_project_search_paths, compile_with_base_dir,
    diagnostic::{Diagnostic, Phase},
    runtime::Interpreter,
    set_project_search_paths,
};
use std::{
    env, fs,
    io::Read,
    path::{Path, PathBuf},
    process::ExitCode,
};

#[derive(Debug, Default)]
struct CliOptions {
    subcommand: Option<String>,
    config_file: Option<String>,
    target_file: Option<String>,
    eval: Option<String>,
    show_tokens: bool,
    show_ast: bool,
    check_only: bool,
}

fn main() -> ExitCode {
    match run_cli() {
        Ok(()) => ExitCode::SUCCESS,
        Err((diagnostic, file_name, source)) => {
            eprintln!("{}", diagnostic.render(&file_name, &source));
            ExitCode::FAILURE
        }
    }
}

fn run_cli() -> Result<(), (Diagnostic, String, String)> {
    let options = parse_cli_args().map_err(|message| {
        (
            Diagnostic::without_span(Phase::Io, message),
            "<command line>".to_owned(),
            String::new(),
        )
    })?;

    // Handle subcommands
    if let Some(ref sub) = options.subcommand {
        match sub.as_str() {
            "new" => {
                let name = options.target_file.as_deref().unwrap_or("my_project");
                handle_new(name).map_err(|e| {
                    (
                        Diagnostic::without_span(Phase::Io, e),
                        "<command line>".to_owned(),
                        String::new(),
                    )
                })?;
                return Ok(());
            }
            "init" => {
                handle_init().map_err(|e| {
                    (
                        Diagnostic::without_span(Phase::Io, e),
                        "<command line>".to_owned(),
                        String::new(),
                    )
                })?;
                return Ok(());
            }
            "test" => {
                let search_dir = options
                    .config_file
                    .as_deref()
                    .or(options.target_file.as_deref());
                handle_test(search_dir.map(Path::new))?;
                return Ok(());
            }
            _ => {}
        }
    }

    // Determine config and target file
    let mut config_opt: Option<ProjectConfig> = None;

    if let Some(ref cfg_path_str) = options.config_file {
        let p = Path::new(cfg_path_str);
        if p.is_file() {
            let loaded = ProjectConfig::load_from_file(p).map_err(|e| {
                (
                    Diagnostic::without_span(Phase::Io, e),
                    cfg_path_str.clone(),
                    String::new(),
                )
            })?;
            set_project_search_paths(loaded.get_search_paths());
            config_opt = Some(loaded);
        }
    }

    let file_name = if let Some(ref tf) = options.target_file {
        tf.clone()
    } else if let Some(ref cfg) = config_opt {
        cfg.entry_path().to_string_lossy().to_string()
    } else if options.eval.is_some() {
        "<eval>".to_owned()
    } else if Path::new("persica.toml").is_file() {
        let loaded = ProjectConfig::load_from_file(Path::new("persica.toml")).map_err(|e| {
            (
                Diagnostic::without_span(Phase::Io, e),
                "persica.toml".to_owned(),
                String::new(),
            )
        })?;
        set_project_search_paths(loaded.get_search_paths());
        let entry = loaded.entry_path().to_string_lossy().to_string();
        config_opt = Some(loaded);
        entry
    } else if Path::new("main.persi").is_file() {
        "main.persi".to_owned()
    } else {
        print_help();
        return Ok(());
    };

    // If no explicit config was specified, search in ancestors of target file
    if config_opt.is_none() && file_name != "<eval>" && file_name != "-" {
        let p = Path::new(&file_name);
        if let Some(loaded) =
            ProjectConfig::find_in_ancestors(p).and_then(|f| ProjectConfig::load_from_file(&f).ok())
        {
            set_project_search_paths(loaded.get_search_paths());
        }
    }

    let source = if let Some(source) = options.eval {
        source
    } else if file_name == "-" {
        let mut source = String::new();
        std::io::stdin()
            .read_to_string(&mut source)
            .map_err(|error| {
                (
                    Diagnostic::without_span(Phase::Io, format!("could not read stdin: {error}")),
                    file_name.clone(),
                    String::new(),
                )
            })?;
        source
    } else {
        fs::read_to_string(&file_name).map_err(|error| {
            (
                Diagnostic::without_span(
                    Phase::Io,
                    format!("could not read '{file_name}': {error}"),
                ),
                file_name.clone(),
                String::new(),
            )
        })?
    };

    let file_path = Path::new(&file_name);
    let base_dir = if file_path.is_file() {
        file_path.parent().map(|p| p.to_path_buf())
    } else {
        None
    };

    let compilation = compile_with_base_dir(&source, base_dir.as_deref())
        .map_err(|error| (error, file_name.clone(), source.clone()))?;

    if options.show_tokens {
        println!("===== TOKENS =====");
        for token in &compilation.tokens {
            println!("{token}");
        }
    }
    if options.show_ast {
        println!("===== AST =====");
        println!("{:#?}", compilation.program);
    }
    if options.check_only {
        println!("check succeeded: {file_name}");
        clear_project_search_paths();
        return Ok(());
    }

    let output = Interpreter::with_stdio()
        .with_base_dir(base_dir)
        .interpret_text(&compilation.program)
        .map_err(|error| (error, file_name.clone(), source.clone()))?;

    clear_project_search_paths();
    if !output.is_empty() {
        print!("{output}");
    }
    Ok(())
}

fn handle_new(name: &str) -> Result<(), String> {
    let proj_dir = PathBuf::from(name);
    if proj_dir.exists() {
        return Err(format!("destination directory '{}' already exists", name));
    }

    fs::create_dir_all(&proj_dir)
        .map_err(|e| format!("could not create project directory: {e}"))?;
    fs::create_dir_all(proj_dir.join("src"))
        .map_err(|e| format!("could not create 'src/' directory: {e}"))?;
    fs::create_dir_all(proj_dir.join("tests"))
        .map_err(|e| format!("could not create 'tests/' directory: {e}"))?;
    fs::create_dir_all(proj_dir.join(".persi_env"))
        .map_err(|e| format!("could not create '.persi_env/' directory: {e}"))?;

    let config = ProjectConfig::new(name, proj_dir.clone());
    fs::write(proj_dir.join("persica.toml"), config.to_toml_string())
        .map_err(|e| format!("could not write persica.toml: {e}"))?;

    let main_code = format!(
        "// {} - Main Entry Point\n\ndef void main() {{\n    print(\"Hello from {}!\");\n}}\n\nmain();\n",
        name, name
    );
    fs::write(proj_dir.join("src").join("main.persi"), main_code)
        .map_err(|e| format!("could not write src/main.persi: {e}"))?;

    let test_code = r#"// Basic Project Test
def void test_example() {
    print("Test passed!");
}

test_example();
"#;
    fs::write(proj_dir.join("tests").join("test_main.persi"), test_code)
        .map_err(|e| format!("could not write tests/test_main.persi: {e}"))?;

    println!("Created Persica project '{name}' successfully!");
    println!("  Project root: {}", proj_dir.display());
    println!("  Environment:  {}", proj_dir.join(".persi_env").display());
    println!("\nTo run your project:");
    println!("  cd {name}");
    println!("  persica");
    Ok(())
}

fn handle_init() -> Result<(), String> {
    let current_dir =
        env::current_dir().map_err(|e| format!("could not get current directory: {e}"))?;
    let toml_path = current_dir.join("persica.toml");
    if toml_path.exists() {
        return Err("a 'persica.toml' already exists in the current directory".to_string());
    }

    let name = current_dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "app".to_string());

    fs::create_dir_all(current_dir.join("src"))
        .map_err(|e| format!("could not create 'src/' directory: {e}"))?;
    fs::create_dir_all(current_dir.join("tests"))
        .map_err(|e| format!("could not create 'tests/' directory: {e}"))?;
    fs::create_dir_all(current_dir.join(".persi_env"))
        .map_err(|e| format!("could not create '.persi_env/' directory: {e}"))?;

    let config = ProjectConfig::new(name.clone(), current_dir.clone());
    fs::write(&toml_path, config.to_toml_string())
        .map_err(|e| format!("could not write persica.toml: {e}"))?;

    let main_persi = current_dir.join("src").join("main.persi");
    if !main_persi.exists() {
        let main_code = format!(
            "// {} - Main Entry Point\n\ndef void main() {{\n    print(\"Hello from {}!\");\n}}\n\nmain();\n",
            name, name
        );
        let _ = fs::write(main_persi, main_code);
    }

    println!(
        "Initialized Persica project in '{}' successfully!",
        current_dir.display()
    );
    println!(
        "  Environment: {}",
        current_dir.join(".persi_env").display()
    );
    Ok(())
}

fn handle_test(target_dir: Option<&Path>) -> Result<(), (Diagnostic, String, String)> {
    let base = target_dir.unwrap_or_else(|| Path::new("."));
    let tests_dir = if base.join("tests").is_dir() {
        base.join("tests")
    } else if base.is_dir() {
        base.to_path_buf()
    } else {
        base.parent()
            .map(|p| p.join("tests"))
            .unwrap_or_else(|| PathBuf::from("tests"))
    };

    let mut test_files = Vec::new();
    if let Ok(entries) = fs::read_dir(&tests_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() && p.extension().is_some_and(|ext| ext == "persi") {
                test_files.push(p);
            }
        }
    }

    if test_files.is_empty() {
        println!(
            "No test files found in '{}' directory.",
            tests_dir.display()
        );
        return Ok(());
    }

    test_files.sort();
    println!("running {} test file(s):", test_files.len());
    let mut passed = 0;
    let mut failed = 0;

    for test_file in &test_files {
        let test_name = test_file.display().to_string();
        let source = fs::read_to_string(test_file).map_err(|e| {
            (
                Diagnostic::without_span(Phase::Io, format!("could not read '{test_name}': {e}")),
                test_name.clone(),
                String::new(),
            )
        })?;

        let base_dir = test_file.parent();
        match persica::run_with_base_dir(&source, base_dir) {
            Ok(_) => {
                println!("  test {test_name} ... ok");
                passed += 1;
            }
            Err(diag) => {
                println!("  test {test_name} ... FAILED");
                eprintln!("{}", diag.render(&test_name, &source));
                failed += 1;
            }
        }
    }

    println!(
        "\ntest result: {}. {} passed; {} failed",
        if failed == 0 { "ok" } else { "FAILED" },
        passed,
        failed
    );

    if failed > 0 {
        std::process::exit(1);
    }
    Ok(())
}

fn parse_cli_args() -> Result<CliOptions, String> {
    let mut options = CliOptions::default();
    let mut positional = Vec::new();
    let mut arguments = env::args().skip(1);

    while let Some(arg) = arguments.next() {
        match arg.as_str() {
            "--tokens" => options.show_tokens = true,
            "--ast" => options.show_ast = true,
            "--check" => options.check_only = true,
            "-e" | "--eval" => {
                if options.eval.is_some() {
                    return Err("only one input source may be provided".to_owned());
                }
                options.eval = Some(
                    arguments
                        .next()
                        .ok_or_else(|| "expected source code after --eval".to_owned())?,
                );
            }
            "-V" | "--version" => {
                println!("Persica {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            "-" => positional.push("-".to_owned()),
            value if value.starts_with('-') => {
                return Err(format!(
                    "unknown option '{value}'\n\nRun 'persica --help' for usage."
                ));
            }
            sub if options.subcommand.is_none()
                && (sub == "new"
                    || sub == "init"
                    || sub == "run"
                    || sub == "test"
                    || sub == "check") =>
            {
                if sub == "check" {
                    options.check_only = true;
                }
                options.subcommand = Some(sub.to_owned());
            }
            val => positional.push(val.to_owned()),
        }
    }

    // Process positional arguments:
    // Cases:
    // 1. persica persica.toml src/main.persi
    // 2. persica persica.toml
    // 3. persica src/main.persi
    // 4. persica new <name>
    if options.subcommand.as_deref() == Some("new") {
        if let Some(name) = positional.into_iter().next() {
            options.target_file = Some(name);
        }
        return Ok(options);
    }

    for pos in positional {
        if pos.ends_with(".toml") || pos == "persica.toml" {
            if options.config_file.is_none() {
                options.config_file = Some(pos);
            } else {
                return Err(format!("unexpected multiple config files: '{pos}'"));
            }
        } else if options.target_file.is_none() {
            options.target_file = Some(pos);
        } else {
            return Err(format!("unexpected argument: '{pos}'"));
        }
    }

    Ok(options)
}

fn print_help() {
    println!(
        "Persica {}\n\nUSAGE:\n    persica [OPTIONS] [FILE]\n    persica [CONFIG.toml] [FILE.persi]\n    persica new <name>\n    persica init\n    persica test\n    persica run [FILE]\n    persica check [FILE]\n    persica [OPTIONS] --eval SOURCE\n\nCOMMANDS:\n    new <name>     Create a new Persica project with .persi_env\n    init           Initialize a Persica project in current directory\n    run [FILE]     Run project or single file\n    test           Discover and run all project tests\n    check [FILE]   Check syntax and types without running\n\nOPTIONS:\n    --tokens       Print lexer output\n    --ast          Print the parsed AST\n    --check        Validate without executing\n    -e, --eval     Execute source passed on the command line\n    -V, --version  Print the version\n    -h, --help     Print this help\n",
        env!("CARGO_PKG_VERSION")
    );
}
