use std::{fs, io::Read, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use image::{self, ImageBuffer, RgbaImage};
use image_processor_plugin::ProcessImageFn;
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
    params_path: String,

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
        params_path,
        plugin_path,
    } = cli;

    let img_input = image::open(&input)
        .with_context(|| format!("Ошибка чтения input файла по пути {}", input))?;

    let img_rgba = img_input.to_rgba8();

    let (width, height) = img_rgba.dimensions();
    let pixels = img_rgba.into_raw();

    let plugin_name = format!("{}_plugin", plugin);
    let lib_name = library_filename(plugin_name);
    let lib_path = PathBuf::from(plugin_path).join(lib_name);

    let params = fs::read_to_string(&params_path)
        .with_context(|| format!("Ошибка чтения файла параметров по пути {}", params_path))?;

    let pixels_out = unsafe {
        let lib = Library::new(lib_path)?;

        let process_image: Symbol<ProcessImageFn> = lib.get(b"process_image")?;

        process_image(width, height, pixels.into(), params.into())
    };

    let pixels_out = pixels_out
        .into_result()
        .map_err(|e| anyhow::anyhow!("{}", e))
        .with_context(|| format!("Ошибка при выполнении алгоритма в плагине {}", plugin))?;

    let img_buffer: RgbaImage = ImageBuffer::from_raw(width, height, pixels_out.into())
        .context("Ошибка преобразования данных в изображение после обработки")?;

    img_buffer
        .save(&output)
        .with_context(|| format!("Ошибка сохранения output файла по пути {}", output))?;

    Ok(())
}
