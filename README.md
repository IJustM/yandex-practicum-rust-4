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
    --params-path=images/blur.json \
    --plugin-path=target/plugins/debug
```

## Плагины

Параметры должны быть указаны в формате `JSON`

### Blur

Параметры:

- radius - радиус размытия (при больших значениях падает производительность из-за выбранного алгоритма)
- iterations - количество итераций по изображению
