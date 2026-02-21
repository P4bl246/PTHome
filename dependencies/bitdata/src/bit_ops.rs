 use std::ops::{BitAnd, BitOr};
 use crate::ops::*;
 pub fn merge<T>(ptr:*mut T, offset:u64, data:T, mask:T, shift:T, byte_len:T)->T
 where T: Copy + SecureSub<Output=T> +
 SecureShift<Output=T>+
 BitAnd<Output = T>+
 BitOr<Output = T>
 {
  unsafe{or(bit_filter(*ptr.offset(offset as isize), mask, shift, true), bit_filter(data, mask, sub(byte_len,shift), false))}
 }
 pub fn bit_filter<T>(bitwise_v:T, v_to_shift:T, v2_to_shift:T, shift_to_right:bool)->T
 where 
 T:SecureShift<Output=T>+
  BitAnd<Output = T>
 {
    return and(bitwise_v, shift(v_to_shift, v2_to_shift, shift_to_right)); 
  }

 pub fn get_as_be_bytes<T>(slice: *const T, slice_size:u64 ,idx_as_be_bytes: usize) -> Option<*const T> {
    let rev_idx = (slice_size as usize).checked_sub(1).unwrap_or(0).checked_sub(idx_as_be_bytes)?;
    Some(unsafe{slice.add(rev_idx)})
}

pub fn get_raw<T>(ptr:*const T, ptr_size:u64, idx:usize)-> Option<*const T>{
   if idx as u64 > sub(ptr_size, 1){return None;}
  unsafe{return Some(ptr.add(idx));}
}