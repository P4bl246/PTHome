use crate::enums::{AllocErr, BytesSlice, DataType, OverFlow};
use crate::bit_ops::bytes_to_bits;
use crate::ops::check_num;
use crate::push_ops::Vars;
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
 pub trait SecureSume{
    type Output;
    fn secure_sum(&self, addend: Self::Output)->Result<Self::Output, AllocErr>;
 }


  pub trait SecureSub{
    type Output;
    fn secure_sub(&self, sustrahend: Self::Output)->Self::Output;
}

 /// # `SecureShift`
  /// A trait that provides a secure way to perform bit shifting operations on various integer types.
  /// The `secure_sh` method takes a shift amount and a boolean indicating the direction of the shift (right or left) 
  /// and returns the result of the shift operation. If the shift amount is greater than or equal to the number of 
  /// bits in the type, it returns 0 to prevent undefined behavior.
  pub trait SecureShift{
    type Output;
    fn secure_sh(&self, shift: Self::Output, to_right:bool)->Self::Output;
}

 pub trait And{
    type Output;
    fn and(&self, val_2:Self::Output)->Self::Output;
 }

  pub trait Or{
    type Output;
    fn or(&self, val_2:Self::Output)->Self::Output;
 }
 
 pub trait UnconstrainedAdd{
    unsafe fn unc_add(self, idx:usize)->Result<Self, AllocErr> where Self: Sized;
 }
 

  pub trait SizeOfT{
    fn size_of_t(&self)->usize;
 }

 /// Trait to optimized the push of data into the bit buffer.
/// Used in DataB struct.
/// This trait is implemented for primitive unsigned integer types (u8, u16, u32, usize).
/// It provides a method to push data into a bit buffer at a specified bit index and size, 
/// with optional overflow management.
pub trait PushOptimized{
   fn push_at_buffer(&self, slf:&mut DataB, bit_index:usize, bit_size:&mut Option<usize>, manage_overflow:Option<OverFlow>)->Result<(), AllocErr>;
   #[inline]
   fn init(&self, slf:&mut DataB, bit_index:usize, bit_size:&mut Option<usize>, manage_overflow:Option<OverFlow>)->Result<Option<Vars>,AllocErr>
  where 
  Self:AsRef<[u8]>+SizeOfT+Copy{
    match Self::downcasting(self, bit_size)?{
      DataType::_N(i)=>{
        let mut var = Vars::new();
     if let Err(i) = var.init(slf, bit_index, bit_size, i, manage_overflow){
       return Err(i);
     }
     if var.get_size_of_push() <= 0{return Ok(None);}
     Ok(Some(var))
    },
      DataType::U8(i)=>{match i.push_at_buffer(slf, bit_index, bit_size, manage_overflow){Ok(_)=>Ok(None), Err(i)=>Err(i)}},
      DataType::U16(i)=>{match i.push_at_buffer(slf, bit_index, bit_size, manage_overflow){Ok(_)=>Ok(None), Err(i)=>Err(i)}},
      DataType::U32(i)=>{match i.push_at_buffer(slf, bit_index, bit_size, manage_overflow){Ok(_)=>Ok(None), Err(i)=>Err(i)}},
      DataType::U64(i)=>{match i.push_at_buffer(slf, bit_index, bit_size, manage_overflow){Ok(_)=>Ok(None), Err(i)=>Err(i)}},
    }
  }
   #[inline]
   fn downcasting(&self, bit_size:&mut Option<usize>)->Result<DataType<&Self>, AllocErr>
   where
   Self: AsRef<[u8]>{
    if let Some(v) = bit_size{
    if check_num(*v, bytes_to_bits(std::mem::size_of_val(self) )?){return Ok(DataType::_N(self));}
    let arr = self.as_ref();
      match *v{
       0..=8=>{
        let data = arr[0];

        return Ok(DataType::U8(data));
      },
       9..=16=>{
        let data = u16::from_be_bytes([arr[0], *arr.get(1).unwrap_or(&0x00)]);

        return Ok(DataType::U16(data));
        },
       17..=32=>{
        let data = u32::from_be_bytes([arr[0], *arr.get(1).unwrap_or(&0x00),
        *arr.get(2).unwrap_or(&0x00), *arr.get(3).unwrap_or(&0x00)]);

        return Ok(DataType::U32(data));
          },
       33..=64=>{
        let data = u64::from_be_bytes([arr[0], *arr.get(1).unwrap_or(&0x00),
        *arr.get(2).unwrap_or(&0x00), *arr.get(3).unwrap_or(&0x00), *arr.get(4).unwrap_or(&0x00),
        *arr.get(5).unwrap_or(&0x00), *arr.get(6).unwrap_or(&0x00), *arr.get(7).unwrap_or(&0x00)]);

        return Ok(DataType::U64(data));
      },
      _=>{return Ok(DataType::_N(self));}
       }
      }
    return Ok(DataType::_N(self));
  }
}


pub trait ImplPush{
    fn push(&self, slf: &mut DataB, vars:&mut Vars)->Result<(), AllocErr>;
}

pub trait ImplWrite{
  fn write<T>(&self, slf:&mut DataB, vars:&Vars, write_strategy:&BytesSlice<T>)->Result<(), AllocErr>;
}

