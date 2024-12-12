use clap::{arg, command, Args, Parser};
use std::fs;
use std::io::{self, BufRead, Write};

#[derive(Parser, Debug)]
struct Cli {
    #[command(flatten)]
    input: Input,
    /// Arguments passed to the script
    arguments: Option<Vec<String>>,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct Input {
    /// Script file path containing the program to execute
    file: Option<String>,
    #[arg(short, long)]
    /// String containing the program to execute
    command: Option<String>,
    #[arg(short, long)]
    /// Read the program from stdin
    stdin: bool,
    #[arg(short, long)]
    /// Run interactivelly
    interactive: bool,
}

fn run_file(path: &String) -> Result<(), exitcode::ExitCode> {
    let program = match fs::read_to_string(path) {
        Ok(program) => program,
        Err(err) => {
            let exit_code = match err.kind() {
                io::ErrorKind::NotFound | io::ErrorKind::PermissionDenied => exitcode::NOINPUT,
                _ => exitcode::IOERR,
            };
            eprintln!("Cannot read input file '{path}': {err}");
            return Err(exit_code);
        }
    };
    run(&program).map_err(|_| exitcode::SOFTWARE)
}

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn run_interactive() -> Result<(), exitcode::ExitCode> {
    // TODO handle multiline statements
    print_prompt();
    for line in io::stdin().lock().lines() {
        match line {
            Ok(line) => {
                run(&line)?;
                print_prompt();
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

//     // For now, just print the tokens.
//     for (Token token : tokens) {
//       System.out.println(token);
//     }
//   }

fn parse_args() -> Result<Cli, exitcode::ExitCode> {
    Cli::try_parse().map_err(|err| {
        let _ = err.print();
        exitcode::USAGE
    })
}

fn try_main() -> Result<(), exitcode::ExitCode> {
    let args = parse_args()?;

    let input = &args.input;
    if let Some(file) = &input.file {
        run_file(&file)?;
    } else if let Some(command) = &input.command {
        run(&command)?;
    } else if input.stdin {
        todo!();
    } else if input.interactive {
        run_interactive()?;
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
