 use crate::ops::{sub};
 use crate::traits::UnconstrainedAdd;
 use crate::enums::AllocErr;
 #[inline]
 pub fn get_as_be_bytes<T>(slice: *const T, slice_size:usize ,idx_as_be_bytes: usize) -> Option<*const T> 
 where *const T: UnconstrainedAdd{
    let rev_idx = (slice_size as usize).checked_sub(1).unwrap_or(0).checked_sub(idx_as_be_bytes)?;
    Some(unsafe{slice.unc_add(rev_idx).ok()?})
}

#[inline]
 pub fn get_as_be_bytes_mut<T>(slice: *mut T, slice_size:usize ,idx_as_be_bytes: usize) -> Option<*mut T> 
 where *mut T: UnconstrainedAdd{
    let rev_idx = (slice_size as usize).checked_sub(1).unwrap_or(0).checked_sub(idx_as_be_bytes)?;
    Some(unsafe{slice.unc_add(rev_idx).ok()?})
}
#[inline]
pub fn get_raw<T>(ptr:*const T, ptr_size:usize, idx:usize)-> Option<*const T>
where *const T: UnconstrainedAdd{
   if idx as usize > sub(ptr_size, 1){return None;}
  unsafe{return Some(ptr.unc_add(idx).ok()?);}
}
#[inline]
pub fn get_raw_mut<T>(ptr:*mut T, ptr_size:usize, idx:usize)-> Option<*mut T>
where *mut T: UnconstrainedAdd{
   if idx as usize > sub(ptr_size, 1){return None;}
  unsafe{return Some(ptr.unc_add(idx).ok()?);}
}

#[inline(always)]
 pub unsafe fn from_raw_parts_unconstrained<'a,T>(ptr:*const T, len:usize)->&'a [T]{
   unsafe{std::mem::transmute((ptr, len))}
 }

 #[inline(always)]
 pub unsafe fn from_raw_parts_mut_unconstrained<'a, T>(ptr:*mut T, len:usize)->&'a mut[T]{
   unsafe{std::mem::transmute((ptr, len))}
 }

 macro_rules! impl_add{
   ($($ty:ty),+)=>{
    $(
      impl UnconstrainedAdd for $ty{
        #[inline(always)]
        unsafe fn unc_add(self, idx:usize)->Result<Self, AllocErr> where Self: Sized{
          return Ok((self as usize).checked_add(idx.checked_mul(std::mem::size_of::<Self>()).ok_or(AllocErr::ArithmeticOverflow)?).ok_or(AllocErr::ArithmeticOverflow)? as $ty);
        }
      }
    )*
   };
 }
 impl_add!(*mut u8, *mut u16, *mut u32, *mut u64, *mut u128, *mut f32, *mut f64, *const u8, *const u16, *const u32, *const u64, *const u128, *const f32, *const f64);
#[macro_export]
macro_rules! derive_add{
   ($($ty:ty),+)=>{
    $(
      impl UnconstrainedAdd for $ty{
        #[inline(always)]
        unsafe fn unc_add(self, idx:usize)->Result<Self, AllocErr> where Self: Sized{
          return Ok((self as usize).checked_add(idx.checked_mul(std::mem::size_of_val(&Self)).ok_or(AllocErr::ArithmeticOverflow)?).ok_or(AllocErr::ArithmeticOverflow)? as $ty);
        }
      }
    )*
   };
 }