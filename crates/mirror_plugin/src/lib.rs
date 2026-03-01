use abi_stable::std_types::{
    RResult::{self, RErr, ROk},
    RString, RVec,
};
use image_processor_plugin::ProcessImageFn;
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

    let width = width as i32;
    let height = height as i32;

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
