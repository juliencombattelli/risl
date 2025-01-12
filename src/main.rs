mod cli;
mod tokenizer;

use std::fs;
use std::io::{self, BufRead, ErrorKind, Write};

use crate::cli::args::Args;
use crate::cli::error::Error;

fn run_file(path: &String) -> Result<(), exitcode::ExitCode> {
    let program = match fs::read_to_string(path) {
        Ok(program) => program,
        Err(err) => {
            let exit_code = match err.kind() {
                ErrorKind::NotFound | ErrorKind::PermissionDenied => exitcode::NOINPUT,
                _ => exitcode::IOERR,
            };
            eprintln!("Cannot read input file '{path}': {err}");
            return Err(exit_code);
        }
    };
    run(&program).map_err(|_| exitcode::SOFTWARE)
}

#[derive(Debug, PartialEq, Eq)]
enum IsInteractive {
    No,
    Yes,
}

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn run_from_stdin(is_interactive: IsInteractive) -> Result<(), exitcode::ExitCode> {
    // TODO handle multiline statements
    if is_interactive == IsInteractive::Yes {
        print_prompt();
    }
    for line in io::stdin().lock().lines() {
        match line {
            Ok(line) => {
                run(&line)?;
                if is_interactive == IsInteractive::Yes {
                    print_prompt();
                }
            }
            Err(err) => {
                eprintln!("Error reading input: {err}");
                return Err(exitcode::IOERR);
            }
        }
    }
    Ok(())
}

fn run(program: &String) -> Result<(), exitcode::ExitCode> {
    println!("INFO: Running program '{}'", program);
    Ok(())
}

// private static void run(String source) {
//     Scanner scanner = new Scanner(source);
//     List<Token> tokens = scanner.scanTokens();
//
//     // For now, just print the tokens.
//     for (Token token : tokens) {
//       System.out.println(token);
//     }
//   }

const USAGE: &'static str = "
Usage:
  risl [-hiv] [ --command=<command> | <file> | --stdin ] [ [--] <arguments>... ]

Options:
  -h --help                 Show this screen.
  -v --version              Show version.
  -i --interactive          Run interactivelly.
  -s --stdin                Read program from the standard input.
  -c --command <command>    Read program from the <command> string.
";

fn try_main() -> Result<(), exitcode::ExitCode> {
    let args = Args::parse().map_err(|err| match err {
        Error::HelpRequested => {
            print!("{}", USAGE);
            exitcode::OK
        }
        _ => {
            println!("Error: {}", err);
            print!("{}", USAGE);
            exitcode::USAGE
        }
    })?;
    println!("{:?}", args);

    if let Some(file) = &args.input_file {
        run_file(&file)?;
    } else if let Some(command) = &args.input_command {
        run(&command)?;
    } else if args.input_is_stdin {
        run_from_stdin(IsInteractive::No)?;
    }

    if args.interactive {
        run_from_stdin(IsInteractive::Yes)?;
    }

    Ok(())
}

fn main() {
    let exit_code = match try_main() {
        Ok(_) => exitcode::OK,
        Err(exit_code) => exit_code,
    };
    std::process::exit(exit_code);
}
