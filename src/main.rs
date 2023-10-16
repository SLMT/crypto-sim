use cli::Cli;

mod cli;

fn main() {
    let mut cli = Cli::new();

    if let Err(message) = cli.run() {
        panic!("Error: {}", message);
    }
}
