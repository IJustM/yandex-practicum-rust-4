use abi_stable::std_types::{RResult, RString, RVec};

// Тип для проверки типизации метода плагинов
pub type ProcessImageFn = unsafe extern "C" fn(
    width: u32,
    height: u32,
    pixels: RVec<u8>,
    params: RString,
) -> RResult<RVec<u8>, RString>;

#[macro_export]
macro_rules! try_to_i32 {
    ($val:expr, $name:expr) => {
        match i32::try_from($val) {
            Ok(v) => v,
            Err(_) => return RErr(RString::from(format!("Ошибка приведения {} к i32", $name))),
        }
    };
}
