use clap::Parser;
use graphviz_rust::{
    attributes::*,
    cmd::Format,
    dot_generator::{graph, id, *},
    dot_structures::*,
    exec,
    printer::PrinterContext,
};
use langs::ConfigParser;
use model::Target;
use rand::Rng;

mod args;
mod langs;
mod model;

fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    let mut targets = match args.language {
        langs::LanguagesConfiguration::Rust(r) => r.parse()?,
    };

    let layer_zero: Vec<_> = targets
        .clone()
        .into_iter()
        .filter(|t| t.dependencies.is_none())
        .collect();

    update_height(&mut targets, 1, layer_zero);

    targets.sort_by_key(|t| t.height);

    let mut g = graph!(strict di id!("id"));

    for t in targets {
        let c = pick_random_color();
        g.add_stmt(Stmt::Node(Node {
            id: node_id!(t.name),
            attributes: vec![attr!("color", c)],
        }));
        if let Some(deps) = t.dependencies {
            for dep in deps {
                g.add_stmt(
                    edge!(node_id!(t.name) => node_id!(dep.name), vec![attr!("color", c)]).into(),
                );
            }
        }
    }

    let graph_svg = exec(
        g,
        &mut PrinterContext::default(),
        vec![(Into::<Format>::into(args.format)).into()],
    )
    .unwrap();

    std::fs::write(args.output, &graph_svg).unwrap();

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

const ALL_COLORS: &[color_name] = &[
    color_name::blue,
    color_name::black,
    color_name::blueviolet,
    color_name::brown,
    color_name::gold,
    color_name::green,
    color_name::magenta,
    color_name::olive,
    color_name::orange,
    color_name::red,
    color_name::purple,
];

fn pick_random_color() -> &'static color_name {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..ALL_COLORS.len());
    let c = &ALL_COLORS[index];
    c
}
