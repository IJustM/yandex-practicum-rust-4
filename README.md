# Проектная работа rust 4

Егоров Дмитрий

## Настройка pre-commit

Включение pre-commit `pre-commit install`

Локальный запуск `pre-commit run --verbose --all-files`

## Запуск

Из корня workspace

```bash
cargo run -p image_processor -- \
    --input=images/input_file.png \
    --output=images/output_file.png \
    --plugin=blur \
    --params=images/blur.txt \
    --plugin-path=target/plugins/debug
```
