use clap::Parser;
use model::TargetKind;

mod args;
mod langs;
mod model;

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    match args.language {
        langs::LanguagesConfiguration::Rust(r) => {
            let mut targets = r.parse()?;

            targets.iter_mut().for_each(|t| {
                if let Some(deps) = &t.dependencies {
                    t.dependencies = Some(
                        deps.iter()
                            .filter(|d| d.kind != TargetKind::Crate)
                            .cloned()
                            .collect(),
                    )
                }
            });

            for target in targets {
                println!("{}", target);
            }
        }
    }

    Ok(())
}
