# Проектная работа rust 4

Егоров Дмитрий

## Настройка pre-commit

Включение pre-commit `pre-commit install`

Локальный запуск `pre-commit run --verbose --all-files`

## Запуск

Из корня workspace

```bash
cargo run -p image_processor -- \
    --input=input_file.png \
    --output=output_file.png \
    --plugin=blur \
    --params=path_to_params \
    --plugin-path=path_to_plugin_folder
```
