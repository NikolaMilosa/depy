use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use graphviz_rust::cmd::Format;

use crate::{
    langs::{ConfigParser, LanguagesConfiguration},
    model::Target,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Output path for dot file
    #[clap(long)]
    pub output: Option<PathBuf>,

    /// Output format
    #[clap(long, value_enum, default_value_t = FormatInt::Svg)]
    pub format: FormatInt,

    /// Indicate a debug print of targets
    #[clap(long, default_value_t = false)]
    pub debug: bool,

    /// Path to the file to be read
    #[clap(long, short)]
    pub path: PathBuf,

    /// The target name to start from
    ///
    /// If the workspace has a lot of dependencies then you can use this feature
    /// to only set this target as a top-level one
    #[clap(long, short, allow_hyphen_values = true)]
    pub top_level: Vec<String>,
}

impl Args {
    pub fn parse_input(&self) -> anyhow::Result<Vec<Target>> {
        let lang: LanguagesConfiguration = self.path.clone().try_into()?;

        let mut targets = lang.parse(self.path.clone())?;
        if !&self.top_level.is_empty() {
            let mut top_levels = vec![];
            for top_level in &self.top_level {
                let target = targets
                    .iter()
                    .find(|t| t.name.eq(top_level))
                    .cloned()
                    .ok_or(anyhow::anyhow!(
                        "Target with name '{}' not found",
                        top_level
                    ))?;
                top_levels.push(target)
            }

            let initial_bag = targets
                .into_iter()
                .filter(|t| !top_levels.iter().any(|tl| tl.eq(t)))
                .collect();

            let mut acc = vec![];
            iterate(&mut acc, top_levels, initial_bag);
            targets = acc
        }

        Ok(targets)
    }
}

fn iterate(acc: &mut Vec<Target>, to_keep: Vec<Target>, bag: Vec<Target>) {
    if to_keep.is_empty() {
        return;
    }

    let mut new_to_keep = vec![];
    let mut new_bag = bag.clone();

    for new_addition in &to_keep {
        if let Some(deps) = &new_addition.dependencies {
            for dep in deps {
                if let Some(pos) = new_bag.iter().position(|t| t.eq(dep)) {
                    let popped = new_bag.remove(pos);
                    new_to_keep.push(popped)
                }
            }
        }
    }

    acc.extend(to_keep);

    iterate(acc, new_to_keep, new_bag)
}

#[derive(ValueEnum, Debug, Clone)]
pub enum FormatInt {
    Bmp,
    Cgimage,
    Canon,
    Dot,
    Gv,
    Xdot,
    Xdot12,
    Xdot14,
    Eps,
    Exr,
    Fig,
    Gd,
    Gd2,
    Gif,
    Gtk,
    Ico,
    Cmap,
    Ismap,
    Imap,
    Cmapx,
    ImapNp,
    CmapxNp,
    Jpg,
    Jpeg,
    Jpe,
    Jp2,
    Json,
    Json0,
    DotJson,
    XdotJson,
    Pdf,
    Pic,
    Pct,
    Pict,
    Plain,
    PlainExt,
    Png,
    Pov,
    Ps,
    Ps2,
    Psd,
    Sgi,
    Svg,
    Svgz,
    Tga,
    Tif,
    Tiff,
    Tk,
    Vml,
    Vmlz,
    Vrml,
    Vbmp,
    Webp,
    Xlib,
    X11,
}

impl From<FormatInt> for Format {
    fn from(value: FormatInt) -> Self {
        match value {
            FormatInt::Bmp => Format::Bmp,
            FormatInt::Dot => Format::Dot,
            FormatInt::Xdot => Format::Xdot,
            FormatInt::Xdot12 => Format::Xdot12,
            FormatInt::Xdot14 => Format::Xdot14,
            FormatInt::Fig => Format::Fig,
            FormatInt::Gif => Format::Gif,
            FormatInt::Jpg => Format::Jpg,
            FormatInt::Jpeg => Format::Jpeg,
            FormatInt::Json => Format::Json,
            FormatInt::Json0 => Format::Json0,
            FormatInt::DotJson => Format::DotJson,
            FormatInt::XdotJson => Format::XdotJson,
            FormatInt::Pdf => Format::Pdf,
            FormatInt::Pic => Format::Pic,
            FormatInt::Pct => Format::Pct,
            FormatInt::Pict => Format::Pict,
            FormatInt::Plain => Format::Plain,
            FormatInt::PlainExt => Format::PlainExt,
            FormatInt::Png => Format::Png,
            FormatInt::Svg => Format::Svg,
            FormatInt::Svgz => Format::Svgz,
            _ => panic!("Unsupported format"),
        }
    }
}
