use crate::enums::{AllocErr, OverFlow};
use crate::bitdata::DataB;

pub trait NativeSliceDataTypes{}

macro_rules! impl_natives_for_slices {
    ($($ty:ty),*) => {
        $(
            impl NativeSliceDataTypes for $ty {
            }
        )*
    };
}

impl_natives_for_slices!(
    &[u8], &[u16], &[u32], &[u64], &[usize], &[i8], &[i16], &[i32], &[i64], &[i128], &[isize], &[char], &[bool], &[f32], &[f64],
    &mut [u8], &mut [u16], &mut [u32], &mut [u64], &mut [usize], &mut [i8], &mut [i16], &mut [i32], &mut [i64],
    &mut [isize], &mut [char], &mut [bool], &mut [f32], &mut [f64]
);
//----------------------------------------------------------------


