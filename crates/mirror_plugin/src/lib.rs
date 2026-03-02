use abi_stable::std_types::{
    RResult::{self, RErr, ROk},
    RString, RVec,
};
use image_processor_plugin::{ProcessImageFn, try_to_i32};
use serde::Deserialize;

#[derive(Deserialize)]
struct MirrorParams {
    horizontal: bool,
    vertical: bool,
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
    let MirrorParams {
        horizontal,
        vertical,
    } = match serde_json::from_str::<MirrorParams>(params.as_str()) {
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

    if horizontal {
        for y in 0..height {
            let row_start = y * width * 4;
            for x in 0..(width / 2) {
                let left_idx = row_start + x * 4;
                let right_idx = row_start + (width - 1 - x) * 4;
                for i in 0..4 {
                    output.swap((left_idx + i) as usize, (right_idx + i) as usize);
                }
            }
        }
    }

    if vertical {
        let row_size = width * 4;
        for y in 0..(height / 2) {
            let top_idx = y * row_size;
            let bottom_idx = (height - 1 - y) * row_size;
            for i in 0..row_size {
                output.swap((top_idx + i) as usize, (bottom_idx + i) as usize);
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
        assert_error("{}", "missing field `horizontal` at line 1 column 2");
        assert_error(
            "{\"horizontal\":true}",
            "missing field `vertical` at line 1 column 19",
        );
        assert_error(
            "{\"horizontal\":\"a\",\"vertical\":false}",
            "invalid type: string \"a\", expected a boolean at line 1 column 17",
        );
        assert_error(
            "{\"horizontal\":123,\"vertical\":false}",
            "invalid type: integer `123`, expected a boolean at line 1 column 17",
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
                "{\"horizontal\":true,\"vertical\":true}".into(),
            ),
            "width",
        );
        assert_error(
            process_image(
                1,
                4294967295,
                vec![].into(),
                "{\"horizontal\":true,\"vertical\":true}".into(),
            ),
            "height",
        );
    }

    #[test]
    fn test_success() {
        let original: Vec<_> = vec![
            vec![1, 1, 1, 0],
            vec![2, 2, 2, 0],
            //
            vec![3, 3, 3, 0],
            vec![4, 4, 4, 0],
        ]
        .into_iter()
        .flatten()
        .collect();

        let res = process_image(
            2,
            2,
            original.clone().into(),
            "{\"horizontal\":false,\"vertical\":false}".into(),
        );
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), RVec::from(original.clone()));

        let res = process_image(
            2,
            2,
            original.clone().into(),
            "{\"horizontal\":true,\"vertical\":false}".into(),
        );
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            RVec::from(
                vec![
                    vec![2, 2, 2, 0],
                    vec![1, 1, 1, 0],
                    //
                    vec![4, 4, 4, 0],
                    vec![3, 3, 3, 0],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
            )
        );

        let res = process_image(
            2,
            2,
            original.clone().into(),
            "{\"horizontal\":false,\"vertical\":true}".into(),
        );
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            RVec::from(
                vec![
                    vec![3, 3, 3, 0],
                    vec![4, 4, 4, 0],
                    //
                    vec![1, 1, 1, 0],
                    vec![2, 2, 2, 0],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
            )
        );
    }
}
