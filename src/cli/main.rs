use clap::{arg, command, Args, Parser};
use std::error::Error;
use std::fs;
use std::io::{self, BufRead, Write};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    /// Run interactivelly after executing the input script, if any
    interactive: bool,
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
}

fn run_file(path: &String) -> Result<(), Box<dyn Error>> {
    let program = fs::read_to_string(path)?;
    run(&program)
}

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn run_interactive() -> Result<(), Box<dyn Error>> {
    // TODO handle multiline statements
    print_prompt();
    for line in io::stdin().lock().lines() {
        let statement = line?;
        run(&statement)?;
        print_prompt();
    }
    Ok(())
}

fn run(program: &String) -> Result<(), Box<dyn Error>> {
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

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let input = &args.input;
    if let Some(file) = &input.file {
        run_file(&file)?;
    } else if let Some(command) = &input.command {
        run(&command)?;
    } else if input.stdin {
        todo!();
    }

    if args.interactive {
        run_interactive()?;
    }

    Ok(())
}
