use std::path::PathBuf;

use graphviz_rust::{
    attributes::*,
    cmd::Format,
    dot_generator::{graph, id, *},
    dot_structures::*,
    exec,
    printer::PrinterContext,
};
use rand::Rng;

use crate::model::Target;

pub struct Drawer {
    format: Format,
    is_debug: bool,
    path: Option<PathBuf>,
}

impl Drawer {
    pub fn new(format: Format, is_debug: bool, path: Option<PathBuf>) -> Self {
        Self {
            format,
            is_debug,
            path,
        }
    }

    pub fn draw(&self, targets: Vec<Target>) -> anyhow::Result<()> {
        match self.is_debug {
            true => self.draw_debug(targets),
            false => self.draw_formatted(targets),
        }
    }

    fn draw_debug(&self, targets: Vec<Target>) -> anyhow::Result<()> {
        for target in targets {
            println!("{}", target);
        }

        Ok(())
    }

    fn draw_formatted(&self, targets: Vec<Target>) -> anyhow::Result<()> {
        let mut g = graph!(strict di id!("id"));

        for t in targets {
            let c = pick_random_color();
            g.add_stmt(Stmt::Node(Node {
                id: node_id!(esc t.name),
                attributes: vec![attr!("color", c)],
            }));
            for dep in t.dependencies {
                g.add_stmt(
                    edge!(node_id!(esc t.name) => node_id!(esc dep), vec![attr!("color", c)])
                        .into(),
                );
            }
        }

        let graph_svg = exec(g, &mut PrinterContext::default(), vec![self.format.into()])?;

        std::fs::write(self.path.clone().unwrap(), &graph_svg).map_err(|e| anyhow::anyhow!(e))
    }
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
