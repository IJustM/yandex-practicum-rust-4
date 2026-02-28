use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use image;
use libloading::{Library, Symbol, library_filename};

#[derive(Debug, Parser)]
#[command(about = "Image processor")]
struct Cli {
    /// Path to source png image
    #[arg(long)]
    input: String,

    /// Path to output png image
    #[arg(long)]
    output: String,

    /// Plugin for processor
    #[arg(long)]
    plugin: String,

    /// Path to params file for plugin
    #[arg(long)]
    params: String,

    /// Path to plugin folder
    #[arg(long, default_value = "target/plugins/debug")]
    plugin_path: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let Cli {
        input,
        output,
        plugin,
        params,
        plugin_path,
    } = cli;

    let img_input = image::open(&input)
        .with_context(|| format!("Ошибка чтения input файла по пути {}", input))?;

    let img_rgba = img_input.to_rgba8();

    let (width, height) = img_rgba.dimensions();
    let raw_pixels = img_rgba.into_raw();

    unsafe {
        let plugin_name = format!("{}_plugin", plugin);
        let lib_name = library_filename(plugin_name);
        let lib_path = PathBuf::from(plugin_path).join(lib_name);

        let lib = Library::new(lib_path)?;

        let run: Symbol<extern "C" fn() -> ()> = lib.get(b"run")?;
        run();
    }

    // img_rgba
    //     .save(&output)
    //     .with_context(|| format!("Ошибка сохранения output файла по пути {}", output))?;

    Ok(())
}
