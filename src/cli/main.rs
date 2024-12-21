use std::fs;
use std::io::{self, BufRead, Write};

/// Utility macro to create Vec<String> from &str slices
macro_rules! str_vec {
    () => (
        $crate::vec::Vec::<String>::new()
    );
    ($($x:expr),+ $(,)?) => (
        [$($x),+].map(String::from).to_vec()
    );
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

#[derive(Debug, PartialEq)]
enum Error {
    HelpRequested,
    MissingArgValue(String),
    UnexpectedArgs(Vec<String>),
    ConflictingArgs(Vec<String>),
}

impl Error {
    fn format_unexpected_args(args: &Vec<String>, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if args.len() == 0 {
            panic!("Error::UnexpectedArgs must contain at least one argument");
        } else if args.len() == 1 {
            write!(f, "unexpected argument '{}' found", args[0])
        } else {
            write!(f, "unexpected arguments found: '{}'", args.join("', '"))
        }
    }

    fn format_conflicting_args(
        args: &Vec<String>,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        if args.len() < 2 {
            panic!("Error::ConflictingArgs must contain at least two arguments");
        } else if args.len() == 2 {
            write!(
                f,
                "the argument '{}' cannot be used with '{}'",
                args[0], args[1]
            )
        } else {
            write!(
                f,
                "the argument '{}' cannot be used with:\n  {}",
                args[0],
                args[1..].join("\n  "),
            )
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::UnexpectedArgs(args) => Self::format_unexpected_args(&args, f),
            Error::ConflictingArgs(args) => Self::format_conflicting_args(&args, f),
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
struct Args {
    input_file: Option<String>,
    input_command: Option<String>,
    input_is_stdin: bool,
    interactive: bool,
    help: bool,
    version: bool,
    script_arguments: Vec<String>,
}

impl Args {
    fn parse() -> Result<Args, Error> {
        Self::parse_from(std::env::args())
    }

    fn parse_from<I, T>(iter: I) -> Result<Args, Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<String> + Clone,
    {
        let args = Args::inner_parse_from(iter)?;
        if args.help {
            // Handle help now with high priority, short cutting any validation
            return Err(Error::HelpRequested);
        }
        args.validate()?;
        Ok(args)
    }

    fn inner_parse_from<I, T>(iter: I) -> Result<Args, Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<String> + Clone,
    {
        let mut args_iter = iter.into_iter();
        args_iter
            .next() // First arg is assumed to be the executable name (not used)
            .expect("Unsupported platform: argument #0 should be the executable name");
        let mut result = Args::default();
        let mut unexpected_args: Vec<String> = vec![];
        let mut end_of_arg_list = false;
        while let Some(arg) = args_iter.next() {
            let arg: String = arg.into();
            if end_of_arg_list {
                // Push current argument if it is not an escape (--)
                if arg.as_str() != "--" {
                    result.script_arguments.push(arg);
                }
                // Push remaining arguments
                result.script_arguments.extend(args_iter.map(|v| v.into()));
                break;
            }
            if arg.starts_with("-") {
                match arg.as_str() {
                    "--" => end_of_arg_list = true,
                    "-c" | "--command" => {
                        if let Some(command) = args_iter.next() {
                            result.input_command = Some(command.into());
                            end_of_arg_list = true;
                        } else {
                            return Err(Error::MissingArgValue(String::from("--command")));
                        }
                    }
                    "-h" | "--help" => result.help = true,
                    "-i" | "--interactive" => result.interactive = true,
                    "-s" | "--stdin" => {
                        result.input_is_stdin = true;
                        end_of_arg_list = true;
                    }
                    "-v" | "--version" => result.version = true,
                    _ => unexpected_args.push(arg),
                }
            } else if result.input_file.is_none() {
                result.input_file = Some(arg);
                end_of_arg_list = true;
            } else {
                result.script_arguments.push(arg);
                end_of_arg_list = true;
            }
        }
        if !unexpected_args.is_empty() {
            return Err(Error::UnexpectedArgs(unexpected_args));
        }
        Ok(result)
    }

    fn validate_no_input_args_conflict(&self) -> Result<(), Error> {
        // Ensure there are not conflict between <input>, --command <command>
        // and --stdin
        // As input options ends the command line and all remaining arguments
        // are forwarded to the called script, there should never be any
        // conflict
        let conflicting_args = match (&self.input_command, &self.input_file, self.input_is_stdin) {
            (None, Some(_), true) => str_vec!["<file>", "--stdin"],
            (Some(_), None, true) => str_vec!["--command=<command>", "--stdin"],
            (Some(_), Some(_), false) => str_vec!["--command=<command>", "<file>"],
            (Some(_), Some(_), true) => str_vec!["--command=<command>", "<file>", "--stdin"],
            (_, _, _) => vec![],
        };
        if !conflicting_args.is_empty() {
            return Err(Error::ConflictingArgs(conflicting_args));
        }
        Ok(())
    }

    fn validate(&self) -> Result<(), Error> {
        self.validate_no_input_args_conflict()?;
        Ok(())
    }
}

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

    // if let Some(file) = &args.arg_file {
    //     run_file(&file)?;
    // } else if let Some(command) = &args.flag_command {
    //     run(&command)?;
    // } else if args.flag_stdin {
    //     todo!();
    // } else if args.flag_interactive {
    //     run_interactive()?;
    // }

    Ok(())
}

fn main() {
    let exit_code = match try_main() {
        Ok(_) => exitcode::OK,
        Err(exit_code) => exit_code,
    };
    std::process::exit(exit_code);
}

#[cfg(test)]
mod cli_tests {
    use super::*;

    mod options {
        use super::*;
        // All options tests here are using Args::inner_parse_from() to bypass
        // validation checks and early help handling

        mod from_stdin {
            use super::*;
            fn expected_result() -> Result<Args, Error> {
                Ok(Args {
                    input_file: None,
                    input_command: None,
                    input_is_stdin: true,
                    interactive: true,
                    help: true,
                    version: true,
                    script_arguments: vec![],
                })
            }
            #[test]
            fn short() {
                let result = Args::inner_parse_from(["risl", "-v", "-h", "-i", "-s"]);
                assert_eq!(result, expected_result());
            }
            #[test]
            fn long() {
                let result = Args::inner_parse_from([
                    "risl",
                    "--version",
                    "--help",
                    "--interactive",
                    "--stdin",
                ]);
                assert_eq!(result, expected_result());
            }
        }

        mod from_command {
            use super::*;
            fn expected_result() -> Result<Args, Error> {
                Ok(Args {
                    input_file: None,
                    input_command: Some(String::from("hello")),
                    input_is_stdin: false,
                    interactive: true,
                    help: true,
                    version: true,
                    script_arguments: vec![],
                })
            }
            #[test]
            fn short() {
                let result = Args::inner_parse_from(["risl", "-v", "-h", "-i", "-c", "hello"]);
                assert_eq!(result, expected_result());
            }
            #[test]
            fn long() {
                let result = Args::inner_parse_from([
                    "risl",
                    "--version",
                    "--help",
                    "--interactive",
                    "--command",
                    "hello",
                ]);
                assert_eq!(result, expected_result());
            }
        }

        mod from_file {
            use super::*;
            fn expected_result() -> Result<Args, Error> {
                Ok(Args {
                    input_file: Some(String::from("file")),
                    input_command: None,
                    input_is_stdin: false,
                    interactive: true,
                    help: true,
                    version: true,
                    script_arguments: vec![],
                })
            }
            #[test]
            fn short() {
                let result = Args::inner_parse_from(["risl", "-v", "-h", "-i", "file"]);
                assert_eq!(result, expected_result());
            }
            #[test]
            fn long() {
                let result = Args::inner_parse_from([
                    "risl",
                    "--version",
                    "--help",
                    "--interactive",
                    "file",
                ]);
                assert_eq!(result, expected_result());
            }
        }
    }

    mod unexpected_args {
        use super::*;
        #[test]
        fn one_unexpected() {
            let result: Result<Args, Error> = Args::parse_from(["risl", "--unexpected"]);
            assert_eq!(result, Err(Error::UnexpectedArgs(str_vec!["--unexpected"])));
        }
        #[test]
        fn multiple_unexpected() {
            let result: Result<Args, Error> = Args::parse_from(["risl", "--unexpected1", "-u2"]);
            assert_eq!(
                result,
                Err(Error::UnexpectedArgs(str_vec!["--unexpected1", "-u2"]))
            );
        }
        #[test]
        fn multiple_unexpected_with_expected() {
            let result: Result<Args, Error> =
                Args::parse_from(["risl", "-i", "--unexpected1", "-u2", "-v"]);
            assert_eq!(
                result,
                Err(Error::UnexpectedArgs(str_vec!["--unexpected1", "-u2"]))
            );
        }
    }

    mod forwarded_arguments {
        use super::*;

        #[test]
        fn from_stdin() {
            let result = Args::inner_parse_from(["risl", "-i", "-v", "-s", "-c", "hello"]);
            assert_eq!(
                result,
                Ok(Args {
                    input_file: None,
                    input_command: None,
                    input_is_stdin: true,
                    interactive: true,
                    help: false,
                    version: true,
                    script_arguments: str_vec!["-c", "hello"],
                })
            );
        }

        #[test]
        fn from_stdin_escaped() {
            let result = Args::inner_parse_from(["risl", "-i", "-v", "-s", "--", "-c", "hello"]);
            assert_eq!(
                result,
                Ok(Args {
                    input_file: None,
                    input_command: None,
                    input_is_stdin: true,
                    interactive: true,
                    help: false,
                    version: true,
                    script_arguments: str_vec!["-c", "hello"],
                })
            );
        }

        #[test]
        fn from_command() {
            let result =
                Args::inner_parse_from(["risl", "-i", "-v", "-c", "hello", "-s", "-u", "hello"]);
            assert_eq!(
                result,
                Ok(Args {
                    input_file: None,
                    input_command: Some(String::from("hello")),
                    input_is_stdin: false,
                    interactive: true,
                    help: false,
                    version: true,
                    script_arguments: str_vec!["-s", "-u", "hello"],
                })
            );
        }

        #[test]
        fn from_command_escaped() {
            let result = Args::inner_parse_from([
                "risl", "-i", "-v", "-c", "hello", "--", "-s", "-u", "hello",
            ]);
            assert_eq!(
                result,
                Ok(Args {
                    input_file: None,
                    input_command: Some(String::from("hello")),
                    input_is_stdin: false,
                    interactive: true,
                    help: false,
                    version: true,
                    script_arguments: str_vec!["-s", "-u", "hello"],
                })
            );
        }

        #[test]
        fn from_file() {
            let args = Args::parse_from(["risl", "file", "hello", "-h"]);
            assert_eq!(
                args,
                Ok(Args {
                    input_file: Some(String::from("file")),
                    input_command: None,
                    input_is_stdin: false,
                    interactive: false,
                    help: false,
                    version: false,
                    script_arguments: str_vec!["hello", "-h"],
                })
            );
        }

        #[test]
        fn from_file_escaped() {
            let args = Args::parse_from(["risl", "file", "--", "hello", "-h"]);
            assert_eq!(
                args,
                Ok(Args {
                    input_file: Some(String::from("file")),
                    input_command: None,
                    input_is_stdin: false,
                    interactive: false,
                    help: false,
                    version: false,
                    script_arguments: str_vec!["hello", "-h"],
                })
            );
        }
    }

    #[test]
    fn missing_input_command() {
        let args = Args::parse_from(["risl", "-c"]);
        assert_eq!(args, Err(Error::MissingArgValue(String::from("--command"))));
    }

    mod conflicts {
        use super::*;
        #[test]
        fn file_stdin() {
            let result = Args {
                input_file: Some(String::from("file")),
                input_command: None,
                input_is_stdin: true,
                interactive: false,
                help: false,
                version: false,
                script_arguments: vec![],
            };
            assert_eq!(
                result.validate(),
                Err(Error::ConflictingArgs(str_vec!["<file>", "--stdin"])),
            );
        }
        #[test]
        fn command_stdin() {
            let result = Args {
                input_file: None,
                input_command: Some(String::from("hello")),
                input_is_stdin: true,
                interactive: false,
                help: false,
                version: false,
                script_arguments: vec![],
            };
            assert_eq!(
                result.validate(),
                Err(Error::ConflictingArgs(str_vec![
                    "--command=<command>",
                    "--stdin"
                ])),
            );
        }
        #[test]
        fn command_file() {
            let result = Args {
                input_file: Some(String::from("file")),
                input_command: Some(String::from("hello")),
                input_is_stdin: false,
                interactive: false,
                help: false,
                version: false,
                script_arguments: vec![],
            };
            assert_eq!(
                result.validate(),
                Err(Error::ConflictingArgs(str_vec![
                    "--command=<command>",
                    "<file>"
                ])),
            );
        }
        #[test]
        fn command_file_stdin() {
            let result = Args {
                input_file: Some(String::from("file")),
                input_command: Some(String::from("hello")),
                input_is_stdin: true,
                interactive: false,
                help: false,
                version: false,
                script_arguments: vec![],
            };
            assert_eq!(
                result.validate(),
                Err(Error::ConflictingArgs(str_vec![
                    "--command=<command>",
                    "<file>",
                    "--stdin"
                ])),
            );
        }
    }
}
