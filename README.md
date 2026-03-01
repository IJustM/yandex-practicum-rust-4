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

Параметры (при больших значениях падает производительность из-за выбранного алгоритма):

```json5
{
    "radius": 2,    // u32; Радиус размытия
    "iterations": 2 // u32; Количество итераций
}
```

### Mirror

Параметры:

```json5
{
    "horizontal": true, // bool; Отобразить по горизонтали
    "vertical": true    // bool; Отобразить по вертикали
}
```
