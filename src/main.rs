use clap::Parser;
use langs::ConfigParser;
use model::{Target, TargetKind};

mod args;
mod langs;
mod model;

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    let mut targets = match args.language {
        langs::LanguagesConfiguration::Rust(r) => {
            let mut targets = r.parse()?;

            targets.iter_mut().for_each(|t| {
                if let Some(deps) = &t.dependencies {
                    let deps: Vec<_> = deps
                        .iter()
                        .filter(|d| d.kind != TargetKind::Crate)
                        .cloned()
                        .collect();
                    t.dependencies = match deps.is_empty() {
                        true => None,
                        false => Some(deps),
                    }
                }
            });

            targets
        }
    };

    let layer_zero: Vec<_> = targets
        .clone()
        .into_iter()
        .filter(|t| t.dependencies.is_none())
        .collect();

    update_height(&mut targets, 1, layer_zero);

    targets.sort_by_key(|t| t.height);

    for target in targets {
        println!("{}", target);
    }

    Ok(())
}

fn update_height(targets: &mut Vec<Target>, height: usize, previous_layer: Vec<Target>) {
    if previous_layer.is_empty() {
        return;
    }
    let mut current_layer = vec![];
    for target in &mut *targets {
        if let Some(deps) = &target.dependencies {
            for dep in deps {
                if previous_layer.iter().any(|t| t.eq(&dep)) {
                    target.height = height;
                    current_layer.push(target.clone());
                    break;
                }
            }
        }
    }

    update_height(targets, height + 1, current_layer);
}
