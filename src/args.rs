use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use graphviz_rust::cmd::Format;

use crate::langs::LanguagesConfiguration;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Output path for dot file
    #[clap(long)]
    pub output: PathBuf,

    /// Output format
    #[clap(long, value_enum, default_value_t = FormatInt::Svg)]
    pub format: FormatInt,

    /// Indicate a debug print of targets
    #[clap(long, default_value_t = false)]
    pub debug: bool,

    #[clap(subcommand)]
    pub language: LanguagesConfiguration,
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
