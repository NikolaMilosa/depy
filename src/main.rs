use clap::Parser;

mod args;
mod langs;
mod model;

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    match args.language {
        langs::LanguagesConfiguration::Rust(r) => {
            let targets = r.parse()?;

            for target in targets {
                println!("{}", target);
            }
        }
    }

    Ok(())
}
