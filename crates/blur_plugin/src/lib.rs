use abi_stable::std_types::{
    RResult::{self, RErr, ROk},
    RString, RVec,
};
use image_processor_plugin::ProcessImageFn;
use serde::Deserialize;

#[derive(Deserialize)]
struct BlurParams {
    radius: i32,
    iterations: i32,
}

#[unsafe(no_mangle)]
pub extern "C" fn process_image(
    width: u32,
    height: u32,
    pixels: RVec<u8>,
    params: RString,
) -> RResult<RVec<u8>, RString> {
    let mut output = pixels.clone();

    let width = width as i32;
    let height = height as i32;

    // Десериализуем JSON параметры
    let BlurParams { radius, iterations } =
        match serde_json::from_str::<BlurParams>(params.as_str()) {
            Ok(p) => p,
            Err(_) => return RErr(RString::from("Ошибка JSON парсинга параметров")),
        };

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

const _: ProcessImageFn = process_image;
