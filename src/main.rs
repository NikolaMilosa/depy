use clap::Parser;
use drawer::Drawer;
use model::Target;

mod args;
mod drawer;
mod langs;
mod model;

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    let mut targets = args.parse_input()?;

    let layer_zero: Vec<_> = targets
        .clone()
        .into_iter()
        .filter(|t| t.dependencies.is_none())
        .collect();

    update_height(&mut targets, 1, layer_zero);

    targets.sort_by_key(|t| t.height);

    let drawer = Drawer::new(args.format.into(), args.debug, args.output);

    drawer.draw(targets)
}

fn update_height(targets: &mut Vec<Target>, height: usize, previous_layer: Vec<Target>) {
    if previous_layer.is_empty() {
        return;
    }
    let mut current_layer = vec![];
    for target in &mut *targets {
        if let Some(deps) = &target.dependencies {
            for dep in deps {
                if previous_layer.iter().any(|t| t.eq(dep)) {
                    target.height = height;
                    current_layer.push(target.clone());
                    break;
                }
            }
        }
    }

    update_height(targets, height + 1, current_layer);
}
