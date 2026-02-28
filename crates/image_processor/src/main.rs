use clap::{Parser, ValueEnum};

#[derive(Debug, ValueEnum, Clone)]
enum ImageProcessorPlugin {
    Mirror,
    Blur,
}

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
    #[arg(long, value_enum)]
    plugin: ImageProcessorPlugin,

    /// Path to params file for plugin
    #[arg(long)]
    params: String,

    /// Path to plugin folder
    #[arg(long, default_value = "target/debug")]
    plugin_path: String,
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}
