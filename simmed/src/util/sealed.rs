use super::size_type::{_8Bit, _16Bit, _32Bit, _64Bit, Size};

pub(crate) trait Sealed {}

macro_rules! impl_sealed {
    ($($T:ty),+) => {
        $(impl Sealed for $T {})+
    };
}

impl_sealed!(
    u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, _8Bit, _16Bit, _32Bit, _64Bit, Size
);
