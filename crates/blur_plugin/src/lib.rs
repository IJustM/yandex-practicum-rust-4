use abi_stable::std_types::{
    RResult::{self, RErr, ROk},
    RString, RVec,
};
use image_processor_plugin::{ProcessImageFn, try_to_i32};
use serde::Deserialize;

#[derive(Deserialize)]
struct BlurParams {
    radius: u32,
    iterations: u32,
}

#[unsafe(no_mangle)]
pub extern "C" fn process_image(
    width: u32,
    height: u32,
    pixels: RVec<u8>,
    params: RString,
) -> RResult<RVec<u8>, RString> {
    let mut output = pixels.clone();

    // Десериализуем JSON параметры
    let BlurParams { radius, iterations } =
        match serde_json::from_str::<BlurParams>(params.as_str()) {
            Ok(p) => p,
            Err(e) => {
                return RErr(RString::from(format!(
                    "Ошибка JSON парсинга параметров: {}",
                    e
                )));
            }
        };

    let width = try_to_i32!(width, "width");
    let height = try_to_i32!(height, "height");
    let radius = try_to_i32!(radius, "radius");

    // Повторяет указанное количество раз
    for _ in 0..iterations {
        let pixels = output.clone();

        for y in 0..height {
            for x in 0..width {
                let mut r = 0u32;
                let mut g = 0u32;
                let mut b = 0u32;
                let mut count = 0u32;

                // Проходим по окну указанного размера
                for ky in -radius..=radius {
                    for kx in -radius..=radius {
                        let py = y + ky;
                        let px = x + kx;

                        // Проверка границ
                        if py >= 0 && py < height && px >= 0 && px < width {
                            let idx = ((py * width + px) * 4) as usize;
                            r += pixels[idx] as u32;
                            g += pixels[idx + 1] as u32;
                            b += pixels[idx + 2] as u32;
                            count += 1;
                        }
                    }
                }

                // Записываем среднее значение
                let out_idx = ((y * width + x) * 4) as usize;
                output[out_idx] = (r / count) as u8;
                output[out_idx + 1] = (g / count) as u8;
                output[out_idx + 2] = (b / count) as u8;
            }
        }
    }

    ROk(output)
}

// Проверка типизации
const _: ProcessImageFn = process_image;

#[cfg(test)]
mod tests_blur_plugin {
    use abi_stable::std_types::{RResult, RString, RVec};

    use crate::process_image;

    #[test]
    fn test_json_parse() {
        fn assert_error(json: &str, error: &str) {
            let res = process_image(0, 0, vec![].into(), json.into());
            assert!(res.is_err());
            assert_eq!(
                res.unwrap_err().to_string(),
                format!("Ошибка JSON парсинга параметров: {}", error)
            );
        }

        assert_error("", "EOF while parsing a value at line 1 column 0");
        assert_error("{}", "missing field `radius` at line 1 column 2");
        assert_error(
            "{\"radius\":1}",
            "missing field `iterations` at line 1 column 12",
        );
        assert_error(
            "{\"radius\":\"a\",\"iterations\":1}",
            "invalid type: string \"a\", expected u32 at line 1 column 13",
        );
    }

    #[test]
    fn test_i32() {
        fn assert_error(res: RResult<RVec<u8>, RString>, name: &str) {
            assert!(res.is_err());
            assert_eq!(
                res.unwrap_err().to_string(),
                format!("Ошибка приведения {} к i32", name)
            );
        }

        assert_error(
            process_image(
                4294967295,
                1,
                vec![].into(),
                "{\"radius\":1,\"iterations\":1}".into(),
            ),
            "width",
        );
        assert_error(
            process_image(
                1,
                4294967295,
                vec![].into(),
                "{\"radius\":1,\"iterations\":1}".into(),
            ),
            "height",
        );
        assert_error(
            process_image(
                1,
                1,
                vec![].into(),
                "{\"radius\":4294967295,\"iterations\":1}".into(),
            ),
            "radius",
        );
    }

    #[test]
    fn test_success() {
        let res = process_image(
            3,
            3,
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                //
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                //
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![255, 255, 255, 0],
            ]
            .into_iter()
            .flatten()
            .collect(),
            "{\"radius\":1,\"iterations\":1}".into(),
        );
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            RVec::from(
                vec![
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    //
                    vec![0, 0, 0, 0],
                    vec![28, 28, 28, 0],
                    vec![42, 42, 42, 0],
                    //
                    vec![0, 0, 0, 0],
                    vec![42, 42, 42, 0],
                    vec![63, 63, 63, 0],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
            )
        );
    }
}
