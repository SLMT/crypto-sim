use cli::Cli;

mod cli;
mod error;
mod option;
mod portofolio;

fn main() {
    let mut cli = Cli::new();

    if let Err(message) = cli.run() {
        panic!("Error: {}", message);
    }
}
