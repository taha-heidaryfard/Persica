use persica::{
    compile_with_base_dir,
    diagnostic::{Diagnostic, Phase},
    runtime::Interpreter,
};
use std::{env, fs, io::Read, process::ExitCode};

#[derive(Debug, Default)]
struct Options {
    file: Option<String>,
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
    let options = parse_options().map_err(|message| {
        (
            Diagnostic::without_span(Phase::Io, message),
            "<command line>".to_owned(),
            String::new(),
        )
    })?;
    let file_name = options.file.unwrap_or_else(|| {
        if options.eval.is_some() {
            "<eval>".to_owned()
        } else {
            "main.persi".to_owned()
        }
    });
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

    let file_path = std::path::Path::new(&file_name);
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
        return Ok(());
    }

    let output = Interpreter::with_stdio()
        .with_base_dir(base_dir)
        .interpret_text(&compilation.program)
        .map_err(|error| (error, file_name.clone(), source.clone()))?;
    if !output.is_empty() {
        print!("{output}");
    }
    Ok(())
}

fn parse_options() -> Result<Options, String> {
    let mut options = Options::default();
    let mut arguments = env::args().skip(1);
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--tokens" => options.show_tokens = true,
            "--ast" => options.show_ast = true,
            "--check" => options.check_only = true,
            "-e" | "--eval" => {
                if options.file.is_some() || options.eval.is_some() {
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
            "-" if options.file.is_none() && options.eval.is_none() => {
                options.file = Some("-".to_owned());
            }
            value if value.starts_with('-') => {
                return Err(format!(
                    "unknown option '{value}'\n\nRun 'persica --help' for usage."
                ));
            }
            value if options.file.is_some() || options.eval.is_some() => {
                return Err(format!("unexpected second input file '{value}'"));
            }
            value => options.file = Some(value.to_owned()),
        }
    }
    Ok(options)
}

fn print_help() {
    println!(
        "Persica {}\n\nUSAGE:\n    persica [OPTIONS] [FILE]\n    persica [OPTIONS] --eval SOURCE\n\nOPTIONS:\n    --tokens       Print lexer output\n    --ast          Print the parsed AST\n    --check        Validate without executing\n    -e, --eval     Execute source passed on the command line\n    -V, --version  Print the version\n    -h, --help     Print this help\n\nUse FILE '-' to read from stdin. If FILE is omitted, Persica reads main.persi.",
        env!("CARGO_PKG_VERSION")
    );
}
