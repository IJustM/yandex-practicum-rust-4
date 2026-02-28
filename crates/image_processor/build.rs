use std::process::Command;

fn main() {
    eprintln!("DEBUG: Начинаю сборку плагина...");

    let plugins = vec!["blur", "mirror"];

    for plugin in plugins {
        let plugin_name = format!("{}_plugin", plugin);

        println!("cargo:rerun-if-changed=../{}/src/lib.rs", plugin_name);

        Command::new("cargo")
            .args(&[
                "build",
                "-p",
                &plugin_name,
                // без смены папки для сборки плагинов происходит блокировка lock файла
                "--target-dir",
                "../../target/plugins",
            ])
            .status()
            .expect("Build plugin error");
    }
}
