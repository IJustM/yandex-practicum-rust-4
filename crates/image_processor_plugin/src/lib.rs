use abi_stable::std_types::{RResult, RString, RVec};

// Тип для проверки типизации метода плагинов
pub type ProcessImageFn = unsafe extern "C" fn(
    width: u32,
    height: u32,
    pixels: RVec<u8>,
    params: RString,
) -> RResult<RVec<u8>, RString>;
