use clap::{arg, Command};

pub fn build_cli() -> Command {
    Command::new("ushort")
        .about("Shortens URLs")
        .author("Damian Ziemba <nazriel@dzfl.pl>")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("create")
                .about("Creates new URL shortener")
                .arg(arg!(<URL> "URL to short"))
                .arg_required_else_help(true),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_cli_instance() {
        let cli = build_cli();
        assert_eq!(cli.get_name(), "ushort");
    }
}
