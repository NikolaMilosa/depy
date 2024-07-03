use clap::Parser;

mod args;
mod langs;
mod model;

fn main() {
    let args = args::Args::parse();

    match args.language {
        langs::LanguagesConfiguration::Rust(_) => {
            println!("Hello, rust!",);
        }
    }
}
