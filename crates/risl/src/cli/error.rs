#[derive(Debug, PartialEq)]
pub enum Error {
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
