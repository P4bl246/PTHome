use crate::enums::AllocErr;
use crate::ops::{sum, sub};
use crate::bit_ops::{bytes_to_bits, bits_to_bytes};
use crate::traits::SizeOfT;
#[inline]
 pub fn byte_overflow(bit:Option<usize>, bit_size:&mut Option<usize>)->Result<bool, AllocErr>{
  if let Some(i) = bit{
    if let Some(j) = bit_size{
      if sum(*j,i)?<bytes_to_bits(bits_to_bytes(*j)?)?{return Ok(false);}
      return Ok(true);
    }else{return Ok(true);}
  }else{return Ok(false);}
 }

  #[inline]
 pub fn t_size<T:SizeOfT + ?Sized>(_value:&T, bit_size:&mut Option<usize>, byte_overflow:bool)->Result<usize, AllocErr>{
    let mut t_size = size_of(_value); // gets the size of the value
    if let Some(b) = bit_size{
      if *b <= 0{return Ok(0);}
      if *b > bytes_to_bits(t_size)?{*b = bytes_to_bits(t_size)?;} //fixed the bit_sized variable, because it cannot be greather than the len of the data
      else{t_size = sub(bits_to_bytes(*b)?,1);} 
      
    }
    //add 1 byte if the bit where start to insert the value is greather than 0
    if byte_overflow{
      t_size = sum(t_size,1)?;
    }
    Ok(t_size)
 }

macro_rules! impl_sizeof {
    ($($ty:ty),*) => {
        $(
            impl SizeOfT for $ty {     
                fn size_of_t(&self) -> usize {
                    std::mem::size_of::<Self>()
                }
            }
        )*
    };
}

impl_sizeof!(u8, u16, u32, usize, u64, u128, i8, i16, i32, i64, isize, i128, f32, f64, bool, char, ());

#[macro_export]
macro_rules! derive_sizeof {
    ($($ty:ty),*) => { 
        $(
            impl SizeOfT for $ty {
                fn size_of_t(&self) -> usize {
                    std::mem::size_of_val(self)
                }
            }
        )*
    };
}
#[inline]
pub fn size_of<T: SizeOfT + ?Sized>(value: &T) -> usize {
    value.size_of_t()
}

