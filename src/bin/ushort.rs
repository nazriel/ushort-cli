use ushort_cli::build_cli;

fn main() {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        Some(("create", data)) => {
            println!("create: {:?}", data)
        }
        _ => println!("Unhandled command given"),
    }
}
