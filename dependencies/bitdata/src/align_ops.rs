 
 use crate::enums::{Aligned, AllocErr, BytesSlice, TypedPtr};
 use crate::ops::{check_num, sub};
 pub fn padding_to_align(t_size:u64, align_with:u64)->Option<u64>{
    if !check_num(align_with, 2){return None;}
    let align = sub(align_with,1);
    let padding = sub(align_with, multiple_of(t_size, align)) & align;
    if padding != 0{
      return Some(padding);
    }
    None
 }
 pub fn sel_aligning_with_memory(t_size:u64, mem_align:Aligned)->Aligned{
   match mem_align{
    Aligned::Bits64=>{
      if t_size <=1{return Aligned::Bits8;}
      else if t_size <=2{return Aligned::Bits16;}
      else if t_size <=4{return Aligned::Bits32;}
    },
    Aligned::Bits32=>{
      if t_size <=1{return Aligned::Bits8;}
      else if t_size <=2{return Aligned::Bits16;}
    },
    Aligned::Bits16=>{if t_size <=1{return Aligned::Bits8;}},

    _=>{}
   }
   mem_align
 }
 pub fn aligned_with_memory(ptr:u64, offset:u64)->Result<Aligned, AllocErr>{
    if check_num(ptr, u64::MAX-offset){return Err(AllocErr::ArithmeticOverflow);}
    let addr = ptr.wrapping_add(offset);
    Ok(aligned_with(addr))
 }
 pub fn transform_pointer<T, U>(sel:&BytesSlice<U>, ptr:*mut T)->TypedPtr<U>{
    match *sel{
      BytesSlice::_8=>TypedPtr::U8(ptr as *mut u8),
      BytesSlice::_16=>TypedPtr::U16(ptr as *mut u16),
      BytesSlice::_32=>TypedPtr::U32(ptr as *mut u32),
      BytesSlice::_64=>TypedPtr::U64(ptr as *mut u64),
      BytesSlice::_N(_)=>TypedPtr::_N(ptr as *mut U)
    }
 }
 /// # `multiple_of`
 /// Checks if a number is a multiple of another number, if it is, it returns the result of
 /// the bitwise AND operation between the two numbers, otherwise it returns the original number.
 /// 
 /// # Arguments
 /// * `num` - The number to be checked.
 /// * `of` - The number to check against.
 /// 
 /// # Example
 /// ```rust
 /// use bitdata::ops::multiple_of;
 /// let result = multiple_of(20, 4);
 /// assert_eq!(result, 4);
 /// let result = multiple_of(20, 3);
 /// assert_eq!(result, 20);
 /// ```
 /// 
 /// # Returns
 /// * `u64` - The result of the bitwise AND operation if `num` is a multiple of `of`, otherwise the original `num`.
 pub fn multiple_of(num:u64, of:u64)->u64{
    num & of
 }

 pub fn aligned_with(value:u64)->Aligned{
    match multiple_of(value, 7){
     0=>{return Aligned::Bits64;},
     4=>{return Aligned::Bits32;},
     2=>{return Aligned::Bits16;},
     _=>{return Aligned::Bits8;},
    }
 }