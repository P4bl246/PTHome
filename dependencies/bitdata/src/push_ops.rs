use crate::enums::*;
use crate::ops::*;
use crate::align_ops::*;
use crate::bit_ops::*;
use std::ops::{BitAnd, BitOr};
use crate::bitdata::DataB;
use std::alloc::{alloc, Layout};
const BYTE_LENGTH:u64 = 8;
//----------------------------------------------------------------
/// Trait to optimized the push of data into the bit buffer.
/// Used in DataB struct.
/// This trait is implemented for primitive unsigned integer types (u8, u16, u32, u64).
/// It provides a method to push data into a bit buffer at a specified bit index and size, 
/// with optional overflow management.
pub trait PushOptimized{
   fn push_at_buffer(&self, slf:&mut DataB, bit_index:u64, bit_size:&mut Option<u64>, manage_overflow:Option<OverFlow>)->Result<(), AllocErr>;
   
   fn init(&self, slf:&mut DataB, bit_index:u64, bit_size:&mut Option<u64>, manage_overflow:Option<OverFlow>)->Result<Option<Vars>,AllocErr>
  where 
  Self:AsRef<[u8]>+SizeOfT+Copy{
    match Self::downcasting(self, bit_size)?{
      DataType::_N(i)=>{
        let mut var = Vars::new();
     if let Err(i) = var.init(slf, bit_index, bit_size, i, manage_overflow){
       return Err(i);
     }
     if var.t_size <= 0{return Ok(None);}
     Ok(Some(var))
    },
      DataType::U8(i)=>{match i.push_at_buffer(slf, bit_index, bit_size, manage_overflow){Ok(_)=>Ok(None), Err(i)=>Err(i)}},
      DataType::U16(i)=>{match i.push_at_buffer(slf, bit_index, bit_size, manage_overflow){Ok(_)=>Ok(None), Err(i)=>Err(i)}},
      DataType::U32(i)=>{match i.push_at_buffer(slf, bit_index, bit_size, manage_overflow){Ok(_)=>Ok(None), Err(i)=>Err(i)}},
      DataType::U64(i)=>{match i.push_at_buffer(slf, bit_index, bit_size, manage_overflow){Ok(_)=>Ok(None), Err(i)=>Err(i)}},
    }
  }
   
   fn downcasting(&self, bit_size:&mut Option<u64>)->Result<DataType<&Self>, AllocErr>
   where
   Self: AsRef<[u8]>{
    if let Some(v) = bit_size{
    if check_num(*v, bytes_to_bits(std::mem::size_of_val(self) as u64)?){return Ok(DataType::_N(self));}
    let arr = self.as_ref();
      match *v{
       0..=8=>{
        #[cfg(target_endian = "little")]
        let data = *arr.get(sub(arr.len(),1)).unwrap_or(&0x00);

        #[cfg(not(target_endian = "little"))]
        let data = arr[0];

        return Ok(DataType::U8(data));
      },
       9..=16=>{
        #[cfg(target_endian = "little")]
        let data = u16::from_be_bytes([*arr.get(sub(arr.len(),1)).unwrap_or(&0x00), *arr.get(sub(arr.len(), 2)).unwrap_or(&0x00)]);
        
        #[cfg(not(target_endian = "little"))]
        let data = u16::from_be_bytes([arr[0], *arr.get(1).unwrap_or(&0x00)]);

        return Ok(DataType::U16(data));
        },
       17..=32=>{
        #[cfg(target_endian = "little")]
         let data = u32::from_be_bytes([*arr.get(sub(arr.len(),1)).unwrap_or(&0x00),
         *arr.get(sub(arr.len(), 2)).unwrap_or(&0x00), *arr.get(sub(arr.len(), 3)).unwrap_or(&0x00),
         *arr.get(sub(arr.len(), 4)).unwrap_or(&0x00)]);
        
        #[cfg(not(target_endian = "little"))]
        let data = u32::from_be_bytes([arr[0], *arr.get(1).unwrap_or(&0x00),
        *arr.get(2).unwrap_or(&0x00), *arr.get(3).unwrap_or(&0x00)]);

        return Ok(DataType::U32(data));
          },
       33..=64=>{

        #[cfg(target_endian = "little")]
        let data = u64::from_be_bytes([*arr.get(sub(arr.len(),1)).unwrap_or(&0x00),
         *arr.get(sub(arr.len(), 2)).unwrap_or(&0x00), *arr.get(sub(arr.len(), 3)).unwrap_or(&0x00),
         *arr.get(sub(arr.len(), 4)).unwrap_or(&0x00), *arr.get(sub(arr.len(), 5)).unwrap_or(&0x00),
         *arr.get(sub(arr.len(), 6)).unwrap_or(&0x00), *arr.get(sub(arr.len(), 7)).unwrap_or(&0x00),
         *arr.get(sub(arr.len(), 8)).unwrap_or(&0x00)]);
        
        #[cfg(not(target_endian = "little"))]
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

#[macro_export]
macro_rules! derive_write{
  ($($ty:ty),+)=>{
    $(
      impl ImplWrite for $ty where $ty:Copy{
        fn write<T>(&self, slf:&mut DataB, vars:&Vars, write_strategy:&BytesSlice<T>)->Result<(),AllocErr>{
          if vars.size == 0{return Ok(());}
          let mut vars2 = vars.clone();
          let raw = *self as *const u8;
          let size = bits_to_bytes(vars.size)?;
          let sz = sub(size, 1);
          unsafe{
          match *write_strategy{
            BytesSlice::_8=>{
              vars2.set_bit_size_val(BYTE_LENGTH);
              let n = div(size, bits_to_bytes(BYTE_LENGTH)?)?; 
              #[cfg(target_endian = "little")]
              for _i in (0..=sub(n,1)as usize).rev(){
                let byte = *raw.add(_i);
                byte.write(slf, &vars2, write_strategy)?;
              }
              #[cfg(not(target_endian = "little"))]
              for _i in 0..=sub(n,1) as usize{
                let byte = *raw.add(_i);
                byte.write(slf, &vars2, write_strategy)?;
              }
              let rest = sub(vars.size, bytes_to_bits(sub(n, 1))?);
              if rest > 0{
                vars2.set_bit_size_val(rest);
                #[cfg(target_endian = "little")]
                let byte = *raw.add(0);
                #[cfg(not(target_endian = "little"))]
                let byte = *raw.get_as_be_bytes(raw, sz, 0).unwrap_or(&0x00u8);
                byte.write(slf, &vars2, write_strategy)?;
              }
              return Ok(());
            },
            BytesSlice::_16=>{
              let r = mult(BYTE_LENGTH, 2)?;
              vars2.set_bit_size_val(r);
              let n = div(size, r)?;
  
              #[cfg(target_endian = "little")]
              for mut _i in (0..=sub(n,1) as usize).rev(){
                let byte = u16::from_be_bytes([*get_as_be_bytes(raw, sz,_i).unwrap_or(raw.add(sz as usize)),
                 *get_as_be_bytes(raw,sz, _i.checked_sub(1).unwrap_or(size as usize)).unwrap_or(&0x00u8)]);
                byte.write(slf, &vars2, write_strategy)?;
                _i-=1;
              }
              #[cfg(not(target_endian = "little"))]
              for mut _i in 0..=n as usize{
                let byte  = u16::from_be_bytes([*get_raw(raw, sz, _i).unwrap_or(0x00), *get_raw(raw, sz, sum(_i, 1)?).unwrap_or(0x00)]);
                byte.write(slf, &vars2, write_strategy)?;
                _i+=1;
              }
              let rest = sub(vars.size, bytes_to_bits(n)?);
              if rest > 0{
                vars2.set_bit_size_val(rest);
                #[cfg(target_endian = "little")]
                let byte = u16::from_be_bytes([*get_raw(raw, sz, 1).unwrap_or(raw.add(0)), *raw.add(0)]);
                #[cfg(not(target_endian = "little"))]
                let byte = u16::from_be_bytes([*ge_as_be_bytes(raw, sz, 0).unwrap_or(&0x00), *get_as_be_bytes(raw, sz, 1).unwrap_or(&0x00)]);
                byte.write(slf, &vars2, write_strategy)?;
              }
              return Ok(());
            },
            BytesSlice::_32=>{
              let r = mult(BYTE_LENGTH, 4)?;
              vars2.set_bit_size_val(r);
              let n = div(size,r)?;
              #[cfg(target_endian = "little")]
              for mut _i in (0..=sub(n,1) as usize).rev(){
                let byte = u32::from_be_bytes([*raw.add(_i), *get_raw(raw, sz, _i.checked_sub(1).unwrap_or(size as usize)).unwrap_or(&0x00u8), 
                *get_raw(raw, sz,_i.checked_sub(2).unwrap_or(size as usize)).unwrap_or(&0x00u8), *get_raw(raw, sz,_i.checked_sub(3).unwrap_or(size as usize)).unwrap_or(&0x00u8)]);
                byte.write(slf, &vars2, write_strategy)?;
                _i-=3;
              }
              #[cfg(not(target_endian = "little"))]
              for mut _i in 0..n as usize{
                let byte = u32::from_be_bytes([raw[_i], *get_raw(raw, sz, sum(_i, 1)?).unwrap_or(0x00), *get_raw(raw, sz, sum(_i, 2)?).unwrap_or(0x00), *get_raw(raw, sz, sum(_i, 3)?).unwrap_or(0x00)]);
                byte.write(slf, &vars2, write_strategy)?;
                _i+=3;
              }
              let rest = sub(vars.size, bytes_to_bits(n)?);
              if rest > 0{
                vars2.set_bit_size_val(rest);
                #[cfg(target_endian = "little")]
                let byte = u32::from_be_bytes([*get_raw(raw, sz, 3).unwrap_or(get_raw(raw, sz,2).unwrap_or(get_raw(raw, sz, 1).unwrap_or(raw.add(0)))),
                *get_raw(raw, sz, 2).unwrap_or(get_raw(raw, sz, 1).unwrap_or(raw.add(0))),*get_raw(raw, sz, 1).unwrap_or(raw.add(0)), *raw.add(0)]);
                #[cfg(not(target_endian = "little"))]
                let byte = u32::from_be_bytes([*raw.add(sz), *get_raw_as_be_bytes(raw, sz, 1).unwrap_or(0x00),
                *get_raw_as_be_bytes(raw, sz, 2).unwrap_or(0x00),*get_raw_as_be_bytes(raw, sz, 3).unwrap_or(0x00)]);

                byte.write(slf, &vars2, write_strategy)?;
              }
              return Ok(());
            },
            BytesSlice::_64=>{
              vars2.set_bit_size_val(mult(BYTE_LENGTH, 8)?);
              let n = div(vars.size, mult(BYTE_LENGTH,8)?)?;
              #[cfg(target_endian = "little")]
              for mut _i in (0..=sub(n,1) as usize).rev(){
                let byte = u64::from_be_bytes([*raw.add(_i), *get_raw(raw, sz, _i.checked_sub(1).unwrap_or(size as usize)).unwrap_or(&0x00u8),
                *get_raw(raw, sz, _i.checked_sub(2).unwrap_or(size as usize)).unwrap_or(&0x00u8),*get_raw(raw, sz, _i.checked_sub(3).unwrap_or(size as usize)).unwrap_or(&0x00u8),
                *get_raw(raw, sz, _i.checked_sub(4).unwrap_or(size as usize)).unwrap_or(&0x00u8),*get_raw(raw, sz, _i.checked_sub(5).unwrap_or(size as usize)).unwrap_or(&0x00u8),
                *get_raw(raw, sz, _i.checked_sub(6).unwrap_or(size as usize)).unwrap_or(&0x00u8),*get_raw(raw, sz, _i.checked_sub(7).unwrap_or(size as usize)).unwrap_or(&0x00u8)]);
                byte.write(slf, &vars2, write_strategy)?;
                _i-=7;
              }
              #[cfg(not(target_endian = "little"))]
              for mut _i in 0..=n as usize{
                let byte = u64::form_be_bytes([raw[_i],*get_raw(raw, sz, sum(_i, 1)?).unwrap_or(0x00),*get_raw(raw, sz, sum(_i, 2)?).unwrap_or(0x00),
                *get_raw(raw, sz, sum(_i, 3)?).unwrap_or(0x00),*get_raw(raw, sz, sum(_i, 4)?).unwrap_or(0x00),*get_raw(raw, sz, sum(_i, 5)?).unwrap_or(0x00),
                *get_raw(raw, sz, sum(_i, 6)?).unwrap_or(0x00),*get_raw(raw, sz, sum(_i, 7)?).unwrap_or(0x00)]);
                byte.write(slf, &vars2, write_strategy)?;
                _i+=7;
              }
              let rest = sub(vars.size, bytes_to_bits(n)?);
              if rest > 0{
                vars2.set_bit_size_val(rest);
                #[cfg(target_endian = "little")]
                let byte = u64::from_be_bytes([*get_raw(raw, sz, 7).unwrap_or(get_raw(raw, sz,6).unwrap_or(get_raw(raw, sz, 5).unwrap_or(
                  get_raw(raw, sz, 4).unwrap_or(get_raw(raw,sz, 3).unwrap_or(get_raw(raw,sz,2).unwrap_or(get_raw(raw, sz, 1).unwrap_or(raw.add(0)))))))),
                *get_raw(raw, sz,6).unwrap_or(get_raw(raw, sz, 5).unwrap_or(
                  get_raw(raw, sz, 4).unwrap_or(get_raw(raw,sz, 3).unwrap_or(get_raw(raw,sz,2).unwrap_or(get_raw(raw, sz, 1).unwrap_or(raw.add(0))))))),
                  *get_raw(raw, sz, 5).unwrap_or(
                  get_raw(raw, sz, 4).unwrap_or(get_raw(raw,sz, 3).unwrap_or(get_raw(raw,sz,2).unwrap_or(get_raw(raw, sz, 1).unwrap_or(raw.add(0)))))),
                  *get_raw(raw, sz, 4).unwrap_or(get_raw(raw,sz, 3).unwrap_or(get_raw(raw,sz,2).unwrap_or(get_raw(raw, sz, 1).unwrap_or(raw.add(0))))),
                  *get_raw(raw,sz, 3).unwrap_or(get_raw(raw,sz,2).unwrap_or(get_raw(raw, sz, 1).unwrap_or(raw.add(0)))),
                  *get_raw(raw,sz,2).unwrap_or(get_raw(raw, sz, 1).unwrap_or(raw.add(0))),
                  *get_raw(raw, sz, 1).unwrap_or(raw.add(0)),
                  *raw.add(0)]);
                #[cfg(not(target_endian = "little"))]
                let byte = u64::from_be_bytes([*raw.add(sz), *get_raw_as_be_bytes(raw, sz, 1).unwrap_or(0x00),  *get_raw_as_be_bytes(raw, sz, 2).unwrap_or(0x00),
                 *get_raw_as_be_bytes(raw, sz, 3).unwrap_or(0x00),  *get_raw_as_be_bytes(raw, sz, 4).unwrap_or(0x00), 
                  *get_raw_as_be_bytes(raw, sz, 5).unwrap_or(0x00),  *get_raw_as_be_bytes(raw, sz, 6).unwrap_or(0x00),  *get_raw_as_be_bytes(raw, sz, 7).unwrap_or(0x00)]);

                byte.write(slf, &vars2, write_strategy)?;
              }
              return Ok(());
            },
            _=>{return Err(AllocErr::UnrecognizedInstruction);}
          }
         }
        }
      }
    )*
  };
}

#[macro_export]
macro_rules! derive_push{
  ($( $ty:ty),+ ) =>{
    $(
      impl ImplPush for $ty where $ty:ImplWrite{
        fn push (&self, slf:&mut DataB, vars:&mut Vars)->Result<(),AllocErr>{
          let s = sel_write_strategy(slf.get_vec(),slf.get_size() as u64,  vars.offset, vars.t_size, vars.remaining_space)?;
            match s.0{
              None =>{},
              Some(i)=>{vars.t_size = i;}
             }
             (*self).write(slf, &vars, &s.1)   
        }
      }
  )*
 };
}
#[macro_export]
macro_rules! derive_pushoptimized{
 ($( $ty:ty),+)=>{
  $( 
    impl PushOptimized for $ty 
    where
    $ty:ImplPush + SizeOfT{
      fn push_at_buffer(&self, slf:&mut DataB, bit_index:u64, bit_size:&mut Option<u64>, manage_overflow:Option<OverFlow>)->Result<(), AllocErr>{
        let mut size = bytes_to_bits(std::mem::size_of_val(self) as u64)?;
        if let Some(v) = bit_size{
          if check_num(*v, bytes_to_bits(std::mem::size_of_val(self) as u64)?){*v=bytes_to_bits(std::mem::size_of_val(self) as u64)?}
            size=*v;
            }
               let arr = (*self as *const $ty) as *const u8;
               let s = bits_to_bytes(size)?;
               unsafe{
             match size{
            0..=8=>{ 
              #[cfg(target_endian = "little")]
              return (*(get_as_be_bytes(arr,s, 0).unwrap_or(&0x00))).push_at_buffer(slf, bit_index, bit_size, manage_overflow);
              #[cfg(not(target_endian = "little"))]
              return *arr.add(0).push_at_buffer(slf, bit_index, bit_size, manage_overflow);
            }
            9..=16=>{
              #[cfg(target_endian = "little")]
              return u16::from_be_bytes([*get_as_be_bytes(arr,s,0).unwrap_or(&0x00), 
              *get_as_be_bytes(arr,s, 1).unwrap_or(&0x00)]).push_at_buffer(slf, bit_index, bit_size, manage_overflow);
              #[cfg(not(target_endian = "little"))]
              return u16::from_be_bytes([*arr.add(0), *arr.add(1).unwrap_or(&0x00)]).push_at_buffer(slf, bit_index, bit_size, manage_overflow);
            },
            17..=32=>{
              #[cfg(target_endian = "little")]
              return u32::from_be_bytes([*get_as_be_bytes(arr,s,0).unwrap_or(&0x00), *get_as_be_bytes(arr,s, 1).unwrap_or(&0x00),
              *get_as_be_bytes(arr,s, 2).unwrap_or(&0x00), *get_as_be_bytes(arr,s, 3).unwrap_or(&0x00)]).push_at_buffer(slf, bit_index, bit_size, manage_overflow);
              #[cfg(not(target_endian = "little"))]
              return u32::from_be_bytes([*arr.add(0), *arr.add(1).unwrap_or(&0x00)
              *arr.add(2).unwrap_or(&0x00), *arr.add(3).unwrap_or(&0x00)]).push_at_buffer(slf, bit_index, bit_size, manage_overflow);
            },
            33..=64=>{
              #[cfg(target_endian = "little")]
              return u64::from_be_bytes([*get_as_be_bytes(arr,s,0).unwrap_or(&0x00), *get_as_be_bytes(arr,s, 1).unwrap_or(&0x00),
              *get_as_be_bytes(arr,s, 2).unwrap_or(&0x00), *get_as_be_bytes(arr,s, 3).unwrap_or(&0x00), *get_as_be_bytes(arr,s, 4).unwrap_or(&0x00),
              *get_as_be_bytes(arr,s,5).unwrap_or(&0x00), *get_as_be_bytes(arr,s, 6).unwrap_or(&0x00),
              *get_as_be_bytes(arr,s, 7).unwrap_or(&0x00)]).push_at_buffer(slf, bit_index, bit_size, manage_overflow);
              #[cfg(not(target_endian = "little"))]
              return u32::from_be_bytes([*arr.add(0), *arr.add(1).unwrap_or(&0x00), *arr.add(2).unwrap_or(&0x00),
              *arr.add(3).unwrap_or(&0x00), *arr.add(4).unwrap_or(&0x00),
              *arr.add(5).unwrap_or(&0x00),*arr.add(6).unwrap_or(&0x00), *arr.add(7).unwrap_or(&0x00)]).push_at_buffer(slf, bit_index, bit_size, manage_overflow);
            },
             _=>{
                  let mut var = Vars::new();
                if let Err(i) = var.init(slf, bit_index, bit_size, self, manage_overflow){
                  return Err(i);
                }
                if var.t_size <= 0{return Ok(());}
                return (*self).push(slf, &mut var); 
             }
        }
       }
      }
    }
  )*
 }; 
}

 impl PushOptimized for u8{
  fn push_at_buffer(&self, slf:&mut DataB, bit_index:u64, bit_size:&mut Option<u64>, manage_overflow:Option<OverFlow>)->Result<(), AllocErr> {
    if let Some(bt) = bit_size{
      if *bt == 1{
        let b = byte_from_bits(bit_index, true)?;
        if check_num(sub(b.0,1),slf.get_size() as u64){return Err(AllocErr::OutOfBounds(slf.get_size() as u64));}
        let mask = shift(0x80u8, b.1.unwrap_or(0) as u8, true);
        
        move_ptr_val(slf, bytes_to_bits(b.0-1)?, b.1.unwrap_or(0))?;
      
        unsafe{
        let value = (mask & *self) | (!mask & *(slf.get_vec().add(b.0 as usize)));
        Vars::fits(slf, value, 1, b.0)?;
        }
      }
    }
    let mut var = Vars::new();
     if let Err(i) = var.init(slf, bit_index, bit_size, self, manage_overflow){
       return Err(i);
     }
     if var.t_size <= 0{return Ok(());}
    
    (*self).push(slf, &mut var)
  }
}
 impl ImplPush for u8{
    fn push(&self, slf: &mut DataB, vars:&mut Vars)->Result<(), AllocErr> {
       (*self).write(slf, &vars, &BytesSlice::<u8>::_8)
       }
  }
 impl ImplWrite for u8{
  #[inline(always)]
   fn write<T>(&self, slf:&mut DataB, vars:&Vars, _write_strategy:&BytesSlice<T>)->Result<(),AllocErr>{
    unsafe{ 
    let ptr = slf.get_vec().add(vars.offset as usize);
    if !vars.no_zero_bit || !vars.byte_overflow{
      if vars.no_bit_size{
        Vars::fits(slf, *self, vars.size, vars.offset)?;
      }
      else{
        let mask = 0xFFu8;
        let shift = vars.size as u8;
        Vars::fits(slf, merge(slf.get_vec(), vars.offset, *self, mask, shift, BYTE_LENGTH as u8), vars.size, vars.offset)?;
      }
    }
    else{
      let mut _j=0;
      let mut _d= [0xFF, 0xFF];
      let mut _masks = [0xFF, 0xFF];
      let bit = vars.bit as u16;
      let shft = BYTE_LENGTH as u16-bit;
      let mask = 0xFFFFu16;
      if vars.no_bit_size{    
        let data = shift(*self as u16, shft, false);
         _d = data.to_be_bytes();
         _masks = (shift(mask,16-bit, false) | shift(mask, 16-shft, true)).to_be_bytes();
        slf.move_ptr(sum(slf.ptr(),BYTE_LENGTH)?);
      }
      else{
        let data = shift(bit_filter((*self as u16) << BYTE_LENGTH, mask, (BYTE_LENGTH-vars.size)as u16, false), bit, true);
         _d = data.to_be_bytes();
        _masks = (shift(mask, 16-bit, false) | shift(shift(mask, bit, true),vars.size as u16, true)).to_be_bytes();
        slf.move_ptr(sum(slf.ptr(),vars.size)?);
      }
        _j+=put_datas(ptr, vars.t_size as usize,&_masks, &_d, _j)?;
        return Vars::ret_overflow(vars.overflow, vars.overflow_len, &_d, _j);
      }
  }
  
    return Ok(());
   }
 }
 
 impl PushOptimized for u16{
  fn push_at_buffer(&self, slf:&mut DataB, bit_index:u64, bit_size:&mut Option<u64>, manage_overflow:Option<OverFlow>)->Result<(), AllocErr> {
    if let Some(v) = bit_size{
      if *v <= 8{
        return ((*self >> bytes_to_bits(1)?) as u8).push_at_buffer(slf, bit_index, bit_size, manage_overflow);
      }
    }
    let mut var = Vars::new();
     if let Err(i) = var.init(slf, bit_index, bit_size, self, manage_overflow){
       return Err(i);
     }
     if var.t_size <= 0{return Ok(());}

     (*self).push(slf, &mut var)
    }
  }
 impl ImplPush for u16{
  fn push(&self, slf:&mut DataB, vars:&mut Vars)->Result<(), AllocErr>{
    let s = sel_write_strategy(slf.get_vec(),slf.get_size() as u64,  vars.offset, vars.t_size, vars.remaining_space)?;
    match s.0{
      None =>{},
      Some(i)=>{vars.t_size = i;}
    }
       (*self).write(slf, &vars, &s.1)
  }
 }
 impl ImplWrite for u16{
  #[inline(always)]
  fn write<T>(&self, slf:&mut DataB, vars:&Vars, write_strategy:&BytesSlice<T>)->Result<(),AllocErr>{
    unsafe{ 
      
      let ptr_l = transform_pointer(write_strategy, slf.get_vec());
    if !vars.no_zero_bit || !vars.byte_overflow{
      if vars.no_bit_size{
        match ptr_l{
          TypedPtr::U8(i)=>{
            let data = (*self).to_be_bytes();
            let last = sum(vars.offset, 1)?;
            copy_data(i, vars.offset, data[0]);
            Vars::fits(slf, data[1], vars.size, last)?;
          },
          TypedPtr::U16(_)=>{
            let offset = div(sum(vars.offset,1)?, 2)?;
            Vars::fits(slf, *self, vars.size, offset)?;
          },
          _=>{return Err(AllocErr::UnrecognizedInstruction);}
        }
        
      }
      else{
        match ptr_l{
          TypedPtr::U8(i)=>{
            let mask = 0xFFu8;
            let shft = vars.size;
            let offs_2 = sum(vars.offset,1)?;
              let data = and(*self, shift(0xFFFF, 16-shft as u16,false)).to_be_bytes();
               copy_data(i, vars.offset, data[0]);
              Vars::fits(slf, merge(slf.get_vec(), offs_2, data[1], mask, shft as u8, mult(BYTE_LENGTH, 2)? as u8), vars.size, offs_2)?;
          },  
          TypedPtr::U16(i)=>{
            let mask = 0xFFFFu16;
            let shift = vars.size as u16;
            let offset = div(sum(vars.offset,1)?, 2)?;
            Vars::fits(slf, merge::<u16>(i, offset, *self, mask, shift, mult(BYTE_LENGTH,2)? as u16), vars.size, offset)?;
          },
          _=>{return Err(AllocErr::UnrecognizedInstruction);}
        } 
      }
    }
    else{
      let j=0;
      let bit = vars.bit as u32;
      let shft = mult(BYTE_LENGTH,2)? as u32-bit;
      let mask = 0xFFFFFFFFu32;
      if vars.no_bit_size{    
        let data = shift(*self as u32, shft, false);
        let masks = shift(mask, 32-bit, false) | shift(mask, 32-shft, true);
        match ptr_l{
          TypedPtr::U8(raw)=>{
            let ptr = raw.add(vars.offset as usize);
            let d = data.to_be_bytes();
            let mask_v = masks.to_be_bytes();
             return return_fn(ptr, vars.t_size, &mask_v,&d, j, slf, vars.overflow, vars.overflow_len, mult(BYTE_LENGTH,2)?);
          },
          TypedPtr::U16(raw)=>{
            let new_t_size = div(vars.t_size,2)?;
            let offset = div(sum(vars.offset,1)?, 2)?;
            let ptr = raw.add(offset as usize);
            let d = data.to_be_bytes();
            let mask_v = masks.to_be_bytes();
            let m = [u16::from_be_bytes([mask_v[0], mask_v[1]]), u16::from_be_bytes([mask_v[2], mask_v[3]])];
            let datas = [u16::from_be_bytes([d[0], d[1]]), u16::from_be_bytes([d[2], d[3]])];
             return return_fn(ptr, new_t_size, &m,&datas, j, slf, vars.overflow, vars.overflow_len, mult(BYTE_LENGTH,2)?);
          },
          TypedPtr::U32(raw)=>{
            let new_t_size = div(vars.t_size, 4)?;
            let offset = div(sum(vars.offset,3)?, 4)?;
            let ptr = raw.add(offset as usize);
             let d = [data];
             let mask_v = [masks];
             return return_fn(ptr, new_t_size, &mask_v,&d, j, slf, vars.overflow, vars.overflow_len, mult(BYTE_LENGTH,2)?);
            },
            _=>{return Err(AllocErr::UnrecognizedInstruction);}
          }
          
        }
        else{
        let data = shift(bit_filter(shift(*self as u32,16, false), mask, (mult(BYTE_LENGTH,4)?-vars.size)as u32, false),bit, true);
        let masks = shift(mask ,32-bit, false) | shift(shift(mask, bit, true), vars.size as u32, true);
         match ptr_l{
          TypedPtr::U8(raw)=>{
            let ptr = raw.add(vars.offset as usize);
            let d = data.to_be_bytes();
            let mask_v = masks.to_be_bytes();
            return return_fn(ptr, vars.t_size, &mask_v,&d, j, slf, vars.overflow, vars.overflow_len, vars.size);
          },
          TypedPtr::U16(raw)=>{
            let new_t_size = div(vars.t_size, 2)?;
            let offset =  div(sum(vars.t_size,1)?, 2)?;
            let ptr = raw.add(offset as usize);
            let d = data.to_be_bytes();
            let mask_v = masks.to_be_bytes();
            let m = [u16::from_be_bytes([mask_v[0], mask_v[1]]), u16::from_be_bytes([mask_v[2], mask_v[3]])];
            let datas = [u16::from_be_bytes([d[0], d[1]]), u16::from_be_bytes([d[2], d[3]])];
            return return_fn(ptr, new_t_size, &m,&datas, j, slf, vars.overflow, vars.overflow_len, vars.size);
          },
          TypedPtr::U32(raw)=>{
            let new_t_size = div(vars.t_size, 4)?;
            let offset = div(sum(vars.offset,3)?, 4)?;
            let ptr = raw.add(offset as usize);
             let d = [data];
             let mask_v = [masks];
             return return_fn(ptr, new_t_size, &mask_v,&d, j, slf, vars.overflow, vars.overflow_len, vars.size);
            },
            _=>{return Err(AllocErr::UnrecognizedInstruction);}
          }
        }  
      }
    }
     Ok(())
    }
 }
 
 impl PushOptimized for u32{
  fn push_at_buffer(&self, slf:&mut DataB, bit_index:u64, bit_size:&mut Option<u64>, manage_overflow:Option<OverFlow>)->Result<(), AllocErr> {
    if let Some(v) = bit_size{
      if *v <= 8{
        return ((*self >> bytes_to_bits(3)?) as u8).push_at_buffer(slf, bit_index, bit_size, manage_overflow);
      }
      else if *v <= 16{
        return ((*self >> bytes_to_bits(2)?) as u16).push_at_buffer(slf, bit_index, bit_size, manage_overflow);
      }
    }
    let mut var = Vars::new();
     if let Err(i) = var.init(slf, bit_index, bit_size, self, manage_overflow){
       return Err(i);
     }
     if var.t_size <= 0{return Ok(());}

     (*self).push(slf, &mut var)
    }
 }
 impl ImplPush for u32{
  fn push(&self, slf:&mut DataB, vars:&mut Vars)->Result<(), AllocErr>{
    let s = sel_write_strategy(slf.get_vec(),slf.get_size() as u64,  vars.offset, vars.t_size, vars.remaining_space)?;
    match s.0{
      None =>{},
      Some(i)=>{vars.t_size = i;}
    }
       (*self).write(slf, &vars, &s.1)
  }
 }
 impl ImplWrite for u32{
  #[inline(always)]
  fn write<T>(&self, slf:&mut DataB, vars:&Vars, write_strategy:&BytesSlice<T>)->Result<(),AllocErr>{
     unsafe{ 
    let ptr_l = transform_pointer(write_strategy, slf.get_vec());
    if !vars.no_zero_bit || !vars.byte_overflow{
      if vars.no_bit_size{
        match ptr_l{
          TypedPtr::U8(i)=>{
            let data = (*self).to_be_bytes();
            let last = sum(vars.offset, 3)?;
            copy_data(i, vars.offset, data[0]);
            copy_data(i, vars.offset+1, data[1]);
            copy_data(i, vars.offset+2, data[2]);
            Vars::fits(slf, data[3], vars.size, last)?;
          },
          TypedPtr::U16(i)=>{
            let d = (*self).to_be_bytes();
            let offset = div(sum(vars.offset, 1)?, 2)?;
            let last =sum(offset, 1)?;
            let data = [u16::from_be_bytes([d[0], d[1]]), u16::from_be_bytes([d[2], d[3]])];
            copy_data(i, offset, data[0]);
            Vars::fits(slf, data[1], vars.size, last)?;
          },
          TypedPtr::U32(_)=>{
            let offset = div(sum(vars.offset,3)?, 4)?;
            Vars::fits(slf, *self, vars.size, offset)?;
          },
          _=>{return Err(AllocErr::UnrecognizedInstruction);}
        }
      }
      else{
        match ptr_l{
          TypedPtr::U8(i)=>{
            let mask = 0xFFu8;
            let shft = vars.size;
            let last = sum(vars.offset, 3)?;
              let data = and(*self, shift(0xFFFFFFFF, (32-shft) as u32, false)).to_be_bytes();
               copy_data(i, vars.offset, data[0]);
               copy_data(i, vars.offset+1, data[1]);
               let s = mult(BYTE_LENGTH, 4)?-shft;
               copy_data(i, vars.offset+2, merge(slf.get_vec(), vars.offset+2, data[2], mask, sub(s, BYTE_LENGTH) as u8,mult(sub(s, BYTE_LENGTH), 2)? as u8));
              Vars::fits(slf, merge(slf.get_vec(), last, data[3], mask, sub(BYTE_LENGTH,s) as u8, BYTE_LENGTH as u8), vars.size, last)?;
          },
          TypedPtr::U16(i)=>{
            let mask = 0xFFFFu16;
            
            let shft = vars.size as u16;
            let offset = div(sum(vars.offset,1)?, 2)?;
            let last = sum(offset, 1)?;
              let d = and(*self, shift(0xFFFFFFFF, (32-shft) as u32, false)).to_be_bytes();
              let data = [u16::from_be_bytes([d[0], d[1]]), u16::from_be_bytes([d[2], d[3]])];
               copy_data(i, offset, data[0]);
              Vars::fits(slf, merge(i, last, data[1], mask, sub(shft,mult(BYTE_LENGTH, 2)? as u16), mult(BYTE_LENGTH, 2)? as u16), vars.size, last)?;
          },
          TypedPtr::U32(i)=>{
            let offset = div(sum(vars.offset,3)?, 4)?;
            let mask = 0xFFFFFFFFu32;
            let shift = vars.size as u32;
            Vars::fits(slf, merge::<u32>(i, offset, *self, mask, shift, mult(BYTE_LENGTH,4)? as u32), vars.size, offset)?;
          },
          _=>{return Err(AllocErr::UnrecognizedInstruction);}
        }  
      }
    }
    else{
      let j=0;
      let bit = vars.bit;
      let shft = mult(BYTE_LENGTH,4)?-bit;
      let mask = 0xFFFFFFFFFFFFFFFFu64;
      if vars.no_bit_size{    
        let data = shift(*self as u64, shft, false);
        let masks = shift(mask, 64-bit, false) | shift(mask, 64-shft, true);
        match ptr_l{
          TypedPtr::U8(raw)=>{
            let ptr = raw.add(vars.offset as usize);
            let d = data.to_be_bytes();
            let mask_v = masks.to_be_bytes();
             return return_fn(ptr, vars.t_size, &mask_v,&d, j, slf, vars.overflow, vars.overflow_len, mult(BYTE_LENGTH,4)?);
          },
          TypedPtr::U16(raw)=>{
            let new_t_size = div(vars.t_size,2)?;
            let offset = div(sum(vars.offset,1)?, 2)?;
            let ptr = raw.add(offset as usize);
            let d = data.to_be_bytes();
            let mask_v = masks.to_be_bytes();
            let m = [u16::from_be_bytes([mask_v[0], mask_v[1]]), u16::from_be_bytes([mask_v[2], mask_v[3]]),u16::from_be_bytes([mask_v[4], mask_v[5]]),
             u16::from_be_bytes([mask_v[6], mask_v[7]])];
            let datas = [u16::from_be_bytes([d[0], d[1]]), u16::from_be_bytes([d[2], d[3]]), u16::from_be_bytes([d[4], d[5]]), u16::from_be_bytes([d[6], d[7]])];
             return return_fn(ptr, new_t_size, &m,&datas, j, slf, vars.overflow, vars.overflow_len, mult(BYTE_LENGTH,4)?);
          },
          TypedPtr::U32(raw)=>{
            let new_t_size = div(vars.t_size,4)?;
            let offset = div(sum(vars.offset,3)?, 4)?;
            let ptr = raw.add(offset as usize);
             let d = data.to_be_bytes();
            let mask_v = masks.to_be_bytes();
            let m = [u32::from_be_bytes([mask_v[0], mask_v[1], mask_v[2], mask_v[3]]), u32::from_be_bytes([mask_v[4], mask_v[5], mask_v[6], mask_v[7]])];
            let datas = [u32::from_be_bytes([d[0], d[1], d[2], d[3]]), u32::from_be_bytes([d[4], d[5], d[6], d[7]])];
             return return_fn(ptr, new_t_size, &m,&datas, j, slf, vars.overflow, vars.overflow_len, mult(BYTE_LENGTH,4)?);
            },
          TypedPtr::U64(raw)=>{
            let new_t_size = div(vars.t_size,8)?;
            let offset = div(sum(vars.offset,7)?, 8)?;
            let ptr = raw.add(offset as usize);
             let d = [data];
             let mask_v = [masks];
             return return_fn(ptr, new_t_size, &mask_v,&d, j, slf, vars.overflow, vars.overflow_len, mult(BYTE_LENGTH,4)?);
          }
          _=>{return Err(AllocErr::UnrecognizedInstruction);}
          }
          
        }
        else{
        let data = shift(bit_filter(shift(*self as u64 , 32, false), mask, (mult(BYTE_LENGTH,8)?-vars.size)as u64, false),bit, true);
        let masks = shift(mask,64-bit, false) | shift(shift(mask, bit, true),vars.size as u64, true);
         match ptr_l{
          TypedPtr::U8(raw)=>{
            let ptr = raw.add(vars.offset as usize);
            let d = data.to_be_bytes();
            let mask_v = masks.to_be_bytes();
             return return_fn(ptr, vars.t_size, &mask_v,&d, j, slf, vars.overflow, vars.overflow_len, vars.size);
          },
          TypedPtr::U16(raw)=>{
            let new_t_size = div(vars.t_size,2)?;
            let offset = div(sum(vars.offset,1)?, 2)?;
            let ptr = raw.add(offset as usize);
            let d = data.to_be_bytes();
            let mask_v = masks.to_be_bytes();
            let m = [u16::from_be_bytes([mask_v[0], mask_v[1]]), u16::from_be_bytes([mask_v[2], mask_v[3]]),u16::from_be_bytes([mask_v[4], mask_v[5]]),
             u16::from_be_bytes([mask_v[6], mask_v[7]])];
            let datas = [u16::from_be_bytes([d[0], d[1]]), u16::from_be_bytes([d[2], d[3]]), u16::from_be_bytes([d[4], d[5]]), u16::from_be_bytes([d[6], d[7]])];
             return return_fn(ptr, new_t_size, &m,&datas, j, slf, vars.overflow, vars.overflow_len, vars.size);
          },
          TypedPtr::U32(raw)=>{
            let new_t_size = div(vars.t_size,4)?;
            let offset = div(sum(vars.offset,3)?, 4)?;
            let ptr = raw.add(offset as usize);
              let d = data.to_be_bytes();
            let mask_v = masks.to_be_bytes();
            let m = [u32::from_be_bytes([mask_v[0], mask_v[1], mask_v[2], mask_v[3]]), u32::from_be_bytes([mask_v[4], mask_v[5], mask_v[6], mask_v[7]])];
            let datas = [u32::from_be_bytes([d[0], d[1], d[2], d[3]]), u32::from_be_bytes([d[4], d[5], d[6], d[7]])];
             return return_fn(ptr, new_t_size, &m,&datas, j, slf, vars.overflow, vars.overflow_len, vars.size);
            },
          TypedPtr::U64(raw)=>{
            let new_t_size = div(vars.t_size,8)?;
            let offset = div(sum(vars.offset,7)?, 8)?;
            let ptr = raw.add(offset as usize);
             let d = [data];
             let mask_v = [masks];
             return return_fn(ptr, new_t_size, &mask_v,&d, j, slf, vars.overflow, vars.overflow_len, vars.size);
          }
          _=>{return Err(AllocErr::UnrecognizedInstruction);}
          }
        }  
      }
     }
     Ok(())
   }  

  }

 impl PushOptimized for u64{
    fn push_at_buffer(&self, slf:&mut DataB, bit_index:u64, bit_size:&mut Option<u64>, manage_overflow:Option<OverFlow>)->Result<(), AllocErr> {
      if let Some(v) = bit_size{
        if *v <= 8{
          return ((*self >> bytes_to_bits(7)?) as u8).push_at_buffer(slf, bit_index, bit_size, manage_overflow);
        }
        else if *v <= 16{
          return ((*self >> bytes_to_bits(6)?) as u16).push_at_buffer(slf, bit_index, bit_size, manage_overflow);
        }
        else if *v <= 32{
          return ((*self >> bytes_to_bits(4)?) as u32).push_at_buffer(slf, bit_index, bit_size, manage_overflow);
        }
      }
    let mut var = Vars::new();
     if let Err(i) = var.init(slf, bit_index, bit_size, self, manage_overflow){
       return Err(i);
     }
     if var.t_size <= 0{return Ok(());}

     (*self).push(slf, &mut var)
    }
 }
 impl ImplPush for u64{
  fn push(&self, slf:&mut DataB, vars:&mut Vars)->Result<(), AllocErr>{
    let s = sel_write_strategy(slf.get_vec(),slf.get_size() as u64,  vars.offset, vars.t_size, vars.remaining_space)?;
    match s.0{
      None =>{},
      Some(i)=>{vars.t_size = i;}
    }
       (*self).write(slf, &vars, &s.1)
  }
 }
 impl ImplWrite for u64{
  #[inline(always)]
  fn write<T>(&self, slf:&mut DataB, vars:&Vars, write_strategy:&BytesSlice<T>)->Result<(),AllocErr>{
    unsafe{ 
    
    let ptr_l = transform_pointer(write_strategy, slf.get_vec());
    if !vars.no_zero_bit || !vars.byte_overflow{
      if vars.no_bit_size{
        match ptr_l{
          TypedPtr::U8(i)=>{
            let data = (*self).to_be_bytes();
            let last = sum(vars.offset, 7)?;
            copy_data(i, vars.offset, data[0]);
            copy_data(i, vars.offset+1, data[1]);
            copy_data(i, vars.offset+2, data[2]);
            copy_data(i, vars.offset+3, data[3]);
            copy_data(i, vars.offset+4, data[4]);
            copy_data(i, vars.offset+5, data[5]);
            copy_data(i, vars.offset+6, data[6]);
            Vars::fits(slf, data[7], vars.size, last)?;
          },
          TypedPtr::U16(i)=>{
            let d = (*self).to_be_bytes();
            let offset = div(sum(vars.offset, 1)?, 2)?;
            let last = sum(offset, 3)?;
            let data = [u16::from_be_bytes([d[0], d[1]]), u16::from_be_bytes([d[2], d[3]]), u16::from_be_bytes([d[4], d[5]]), u16::from_be_bytes([d[6], d[7]])];
            copy_data(i, offset, data[0]);
              copy_data(i, offset+1, data[1]);
              copy_data(i, offset+2, data[2]);
            Vars::fits(slf, data[3], vars.size, last)?;
          },
          TypedPtr::U32(i)=>{
            let d = (*self).to_be_bytes();
            let offset = div(sum(vars.offset, 3)?, 4)?;
            let last = sum(offset, 1)?;
            let data = [u32::from_be_bytes([d[0], d[1], d[2], d[3]]), u32::from_be_bytes([d[4], d[5], d[6], d[7]])];
            copy_data(i, offset, data[0]);
            Vars::fits(slf, data[1], vars.size, last)?;
          },
          TypedPtr::U64(_)=>{
            let offset = div(sum(vars.offset,7)?, 8)?;
            Vars::fits(slf, *self, vars.size, offset)?;
          },
          _=>{return Err(AllocErr::UnrecognizedInstruction);}
        }
      }
      else{
        match ptr_l{
          TypedPtr::U8(i)=>{
            let mask = 0xFFu8;
            let shft = vars.size as u8;
            let last = sum(vars.offset, 7)?;
              let data = and(*self, shift(0xFFFFFFFFFFFFFFFF, 64-shft as u64,false)).to_be_bytes();
               copy_data(i, vars.offset, data[0]);
               copy_data(i, vars.offset+1, data[1]);
               copy_data(i, vars.offset+2, data[2]);
               copy_data(i, vars.offset+3, data[3]);
               let s = (mult(BYTE_LENGTH, 8)? as u8)-shft;
               copy_data(i, vars.offset+4, merge(slf.get_vec(), vars.offset+4, data[4], mask, sub(s, mult(BYTE_LENGTH, 3)? as u8), mult(sub(s as u64, mult(BYTE_LENGTH, 3)? ),2)? as u8));
               copy_data(i, vars.offset+5, merge(slf.get_vec(), vars.offset+5, data[5], mask,  sub( mult(BYTE_LENGTH, 3)? as u8, s), BYTE_LENGTH as u8));
               copy_data(i, vars.offset+6, merge(slf.get_vec(), vars.offset+6, data[6], mask, sub(sub( mult(BYTE_LENGTH, 3)? as u8, s),BYTE_LENGTH as u8), BYTE_LENGTH as u8));
              Vars::fits(slf, merge(slf.get_vec(), last, data[7], mask, sub(sub(s as u64, mult(BYTE_LENGTH, 3)?), mult(BYTE_LENGTH, 2)? )as u8, BYTE_LENGTH as u8), vars.size, last)?;
          },
          TypedPtr::U16(i)=>{
            let mask = 0xFFFFu16;
            let shft = vars.size as u16;
            let offset = div(sum(vars.offset,1)?, 2)?;
            let last = sum(offset, 3)?;
              let d = and(*self, shift(0xFFFFFFFFFFFFFFFF, 64-shft as u64,false)).to_be_bytes();
              let data = [u16::from_be_bytes([d[0], d[1]]), u16::from_be_bytes([d[2], d[3]]), u16::from_be_bytes([d[4], d[5]]), u16::from_be_bytes([d[6], d[7]])];
               copy_data(i, offset, data[0]);
               copy_data(i, offset+1, data[1]);
               let s = (mult(BYTE_LENGTH, 8)? as u16)-shft;
               copy_data(i, offset+2, merge(i, offset+2, data[2], mask, sub(mult(BYTE_LENGTH, 4)? as u16,s), mult(BYTE_LENGTH, 2)? as u16));
              Vars::fits(slf, copy_data(i, last, merge(i, vars.offset, data[3], mask, sub(mult(BYTE_LENGTH, 2)? as u16, s), mult(BYTE_LENGTH, 2)? as u16)), vars.size, last)?;
          },
          TypedPtr::U32(i)=>{
            let mask = 0xFFFFFFFFu32;
            let shft = vars.size as u32;
            let offset = div(sum(vars.offset,3)?, 4)?;
            let last = sum(offset, 1)?;
            let d = and(*self, shift(0xFFFFFFFFFFFFFFFF, 64-shft as u64,false)).to_be_bytes();
            let data = [u32::from_be_bytes([d[0], d[1], d[2], d[3]]), u32::from_be_bytes([d[4], d[5], d[6], d[7]])];
              copy_data(i, offset, data[0]);
              let s = (mult(BYTE_LENGTH, 8)? as u32)-shft;
              Vars::fits(slf, merge::<u32>(slf.get_vec() as *mut u32, last, data[1], mask, sub(mult(BYTE_LENGTH, 4)? as u32,s), mult(BYTE_LENGTH,4)? as u32), vars.size, last)?;
          },
          TypedPtr::U64(i)=>{
            let mask = 0xFFFFFFFFFFFFFFFFu64;
            let offset = div(sum(vars.offset,7)?, 8)?;
            let shift = vars.size as u64;
            Vars::fits(slf, merge::<u64>(i, offset, *self, mask, shift, mult(BYTE_LENGTH,8)? as u64), vars.size, offset)?;
          },
          _=>{return Err(AllocErr::UnrecognizedInstruction);}
        }  
      }
    }
    else{
      let j=0;
      let bit = vars.bit as u64;
      let shft = bit;
      let mask = 0xFFFFFFFFFFFFFFFFu64;
      if vars.no_bit_size{    
        let data = shift(*self, shft, true);
        let masks =  shift(mask, 64-bit, false) | shift(mask,64-(BYTE_LENGTH-shft), true); 
        match ptr_l{
          TypedPtr::U8(raw)=>{
            let ptr = raw.add(vars.offset as usize);
            let d_temp = data.to_be_bytes();
            let mask_v_temp = masks.to_be_bytes();
            let carry = shift(*self, BYTE_LENGTH-bit, false) as u8;
            let  d = [d_temp[0], d_temp[1], d_temp[2], d_temp[3], d_temp[4], d_temp[5], d_temp[6], d_temp[7], carry];
            let mask_v = [mask_v_temp[0], mask_v_temp[1], mask_v_temp[2], mask_v_temp[3], mask_v_temp[4], mask_v_temp[5], mask_v_temp[6], 0x00, mask_v_temp[7]];
             return return_fn(ptr, vars.t_size, &mask_v,&d, j, slf, vars.overflow, vars.overflow_len, mult(BYTE_LENGTH,8)?);
          },
          TypedPtr::U16(raw)=>{
            let new_t_size = div(vars.t_size,2)?;
            let offset = div(sum(vars.offset,1)?, 2)?;
            let ptr = raw.add(offset as usize);
            let d = data.to_be_bytes();
            let mask_v = masks.to_be_bytes();
            let carry = shift(shift(*self & 0xFF, BYTE_LENGTH-bit, false),BYTE_LENGTH, false) as u16;
            let m = [u16::from_be_bytes([mask_v[0], mask_v[1]]),u16::from_be_bytes([mask_v[2], mask_v[3]]), 
            u16::from_be_bytes([mask_v[4], mask_v[5]]), u16::from_be_bytes([mask_v[6], 0x00]), shift(mask_v[7] as u16, BYTE_LENGTH as u16, false) & shift(0xFFFF,shft,true) as u16];
            let datas = [u16::from_be_bytes([d[0], d[1]]),u16::from_be_bytes([d[2], d[3]]), 
            u16::from_be_bytes([d[4], d[5]]), u16::from_be_bytes([d[6], d[7]]), carry ]; 
             return return_fn(ptr, new_t_size, &m,&datas, j, slf, vars.overflow, vars.overflow_len, mult(BYTE_LENGTH,8)?);
          },
          TypedPtr::U32(raw)=>{
            let new_t_size = div(vars.t_size,4)?;
            let offset = div(sum(vars.offset,3)?, 4)?;
            let ptr = raw.add(offset as usize);
            let d = data.to_be_bytes();
            let mask_v = masks.to_be_bytes();
            let carry = shift(shift(*self & 0xFF,BYTE_LENGTH-bit, false) as u32, mult(BYTE_LENGTH,4)? as u32, false);
            let m = [u32::from_be_bytes([mask_v[0], mask_v[1],mask_v[2], mask_v[3]]), 
            u32::from_be_bytes([mask_v[4], mask_v[5], mask_v[6], 0x00]), shift(mask_v[7] as u32, mult(BYTE_LENGTH, 4)? as u32, false) & shift(0xFFFFFFFF,shft as u32, true)];
            let datas = [u32::from_be_bytes([d[0], d[1], d[2], d[3]]), 
            u32::from_be_bytes([d[4], d[5],d[6], d[7]]), carry];
             return return_fn(ptr, new_t_size, &m,&datas, j, slf, vars.overflow, vars.overflow_len, mult(BYTE_LENGTH,8)?);
            },
          TypedPtr::U64(raw)=>{
            let new_t_size = div(vars.t_size,8)?;
            let offset = div(sum(vars.offset,7)?, 8)?;
            let ptr = raw.add(offset as usize);
            let carry = shift(shift(*self & 0xFF, BYTE_LENGTH-bit, false) as u64, mult(BYTE_LENGTH,8)?, false);
             let d = [data, carry];
             let t_mask = masks.to_be_bytes();
             let mask_v = [u64::from_be_bytes([t_mask[0], t_mask[1], t_mask[2], t_mask[3], t_mask[4], t_mask[5], t_mask[6], 0x00]),
             shift(t_mask[7] as u64, mult(BYTE_LENGTH, 8)?, false) & shift(0xFFFFFFFFFFFFFFFF,shft, true)];
             return return_fn(ptr, new_t_size, &mask_v,&d, j, slf, vars.overflow, vars.overflow_len, mult(BYTE_LENGTH,8)?);
          }
          _=>{return Err(AllocErr::UnrecognizedInstruction);}
        }
  
        }
        else{
        let data = shift(bit_filter(*self, mask, (mult(BYTE_LENGTH,8)?-vars.size)as u64, false),shft, true);

        let masks = shift(mask,64-bit, false) | shift(shift(mask, 64-vars.size as u64, true),bit, true);
        let last_mask = shift(mask, vars.size-(64-bit), true);
         match ptr_l{
          TypedPtr::U8(raw)=>{
            let ptr = raw.add(vars.offset as usize);
            let d_temp = data.to_be_bytes();
            let mask_v_temp = masks.to_be_bytes();                                                                        
            let carry = shift(shift(*self,64-vars.size, true),BYTE_LENGTH-bit, false) as u8;
            let  d = [d_temp[0], d_temp[1], d_temp[2], d_temp[3], d_temp[4], d_temp[5], d_temp[6], d_temp[7], carry];
            let mask_v = [mask_v_temp[0], mask_v_temp[1], mask_v_temp[2], mask_v_temp[3], mask_v_temp[4], mask_v_temp[5], mask_v_temp[6], mask_v_temp[7], shift(last_mask,64-BYTE_LENGTH, true) as u8];
             return return_fn(ptr, vars.t_size, &mask_v,&d, j, slf, vars.overflow, vars.overflow_len, mult(BYTE_LENGTH,8)?-vars.size);
          },
          TypedPtr::U16(raw)=>{
            let new_t_size = div(vars.t_size,2)?;
            let offset = div(sum(vars.offset,1)?, 2)?;
            let ptr = raw.add(offset as usize);
            let d = data.to_be_bytes();
            let mask_v = masks.to_be_bytes();
            let carry = shift(shift(shift(*self,64-vars.size, true),BYTE_LENGTH-bit, false) as u16,BYTE_LENGTH as u16, false);
            let m = [u16::from_be_bytes([mask_v[0], mask_v[1]]),u16::from_be_bytes([mask_v[2], mask_v[3]]), 
            u16::from_be_bytes([mask_v[4], mask_v[5]]), u16::from_be_bytes([mask_v[6], mask_v[7]]), shift(last_mask,64-mult(BYTE_LENGTH, 2)?, true) as u16];
            let datas = [u16::from_be_bytes([d[0], d[1]]),u16::from_be_bytes([d[2], d[3]]), 
            u16::from_be_bytes([d[4], d[5]]), u16::from_be_bytes([d[6], d[7]]), carry];
             return return_fn(ptr, new_t_size, &m,&datas, j, slf, vars.overflow, vars.overflow_len, vars.size);
          },
          TypedPtr::U32(raw)=>{
            let new_t_size = div(vars.t_size,4)?;
            let offset = div(sum(vars.offset,3)?, 4)?;
            let ptr = raw.add(offset as usize);
            let d = data.to_be_bytes();
            let mask_v = masks.to_be_bytes();
            let carry = shift(shift(shift(*self,64-vars.size, true),BYTE_LENGTH-bit, false) as u32, mult(BYTE_LENGTH,4)? as u32, false);
            let m = [u32::from_be_bytes([mask_v[0], mask_v[1],mask_v[2], mask_v[3]]), 
            u32::from_be_bytes([mask_v[4], mask_v[5], mask_v[6],mask_v[7]]), shift(last_mask, 64 - mult(BYTE_LENGTH,4)?, true) as u32];
            let datas = [u32::from_be_bytes([d[0], d[1], d[2], d[3]]), 
            u32::from_be_bytes([d[4], d[5],d[6], d[7]]), carry];
             return return_fn(ptr, new_t_size, &m,&datas, j, slf, vars.overflow, vars.overflow_len, vars.size);
            },
          TypedPtr::U64(raw)=>{
            let new_t_size = div(vars.t_size,8)?;
            let offset = div(sum(vars.offset,7)?, 8)?;
            let ptr = raw.add(offset as usize);
            let carry = shift(shift(shift(*self,64-vars.size, true),BYTE_LENGTH-bit, false) as u64, mult(BYTE_LENGTH,8)?, false);
             let d = [data, carry];
             let t_mask = masks.to_be_bytes();
             let mask_v = [u64::from_be_bytes([t_mask[0], t_mask[1], t_mask[2], t_mask[3], t_mask[4], t_mask[5], t_mask[6], t_mask[7]]),
             shift(last_mask,64 - mult(BYTE_LENGTH,8)?, true) as u64];
             return return_fn(ptr, new_t_size, &mask_v,&d, j, slf, vars.overflow, vars.overflow_len,vars.size);
          }
          _=>{return Err(AllocErr::UnrecognizedInstruction);}
        }
        }  
      }
     }
     Ok(())
  }
 }

 impl PushOptimized for usize{
    fn push_at_buffer(&self, slf:&mut DataB, bit_index:u64, bit_size:&mut Option<u64>, manage_overflow:Option<OverFlow>)->Result<(), AllocErr> {
      let value = *self as u64;
      return value.push_at_buffer(slf, bit_index, bit_size, manage_overflow);
    }
 }

 impl PushOptimized for i8{
    fn push_at_buffer(&self, slf:&mut DataB, bit_index:u64, bit_size:&mut Option<u64>, manage_overflow:Option<OverFlow>)->Result<(), AllocErr> {
      let value = *self as u8;
      return value.push_at_buffer(slf, bit_index, bit_size, manage_overflow);
    }
 }

 impl PushOptimized for i16{
    fn push_at_buffer(&self, slf:&mut DataB, bit_index:u64, bit_size:&mut Option<u64>, manage_overflow:Option<OverFlow>)->Result<(), AllocErr> {
      let value = *self as u16;
      return value.push_at_buffer(slf, bit_index, bit_size, manage_overflow);
    }
 }

 impl PushOptimized for i32{
    fn push_at_buffer(&self, slf:&mut DataB, bit_index:u64, bit_size:&mut Option<u64>, manage_overflow:Option<OverFlow>)->Result<(), AllocErr> {
      let value = *self as u32;
      return value.push_at_buffer(slf, bit_index, bit_size, manage_overflow);
    }
 }

 impl PushOptimized for i64{
    fn push_at_buffer(&self, slf:&mut DataB, bit_index:u64, bit_size:&mut Option<u64>, manage_overflow:Option<OverFlow>)->Result<(), AllocErr> {
      let value = *self as u64;
      return value.push_at_buffer(slf, bit_index, bit_size, manage_overflow);
    }
 }

 impl PushOptimized for isize{
    fn push_at_buffer(&self, slf:&mut DataB, bit_index:u64, bit_size:&mut Option<u64>, manage_overflow:Option<OverFlow>)->Result<(), AllocErr> {
      let value = *self as u64;
      return value.push_at_buffer(slf, bit_index, bit_size, manage_overflow);
    }
  }

 impl PushOptimized for char{
    fn push_at_buffer(&self, slf:&mut DataB, bit_index:u64, bit_size:&mut Option<u64>, manage_overflow:Option<OverFlow>)->Result<(), AllocErr> {
      let value = *self as u32;
      return value.push_at_buffer(slf, bit_index, bit_size, manage_overflow);
    }
  }

 impl PushOptimized for bool{
    fn push_at_buffer(&self, slf:&mut DataB, bit_index:u64, bit_size:&mut Option<u64>, manage_overflow:Option<OverFlow>)->Result<(), AllocErr> {
      let value = if *self {1u8} else {0u8};
      return value.push_at_buffer(slf, bit_index, bit_size, manage_overflow);
    }
  }

 impl PushOptimized for (){
    fn push_at_buffer(&self, _slf:&mut DataB, _bit_index:u64, _bit_size:&mut Option<u64>, _manage_overflow:Option<OverFlow>)->Result<(), AllocErr> {
      Ok(())
    }
 }
 
 impl PushOptimized for f32{
    fn push_at_buffer(&self, slf:&mut DataB, bit_index:u64, bit_size:&mut Option<u64>, manage_overflow:Option<OverFlow>)->Result<(), AllocErr> {
      let value = self.to_bits();
      return value.push_at_buffer(slf, bit_index, bit_size, manage_overflow);
    }
 }

 impl PushOptimized for f64{
    fn push_at_buffer(&self, slf:&mut DataB, bit_index:u64, bit_size:&mut Option<u64>, manage_overflow:Option<OverFlow>)->Result<(), AllocErr> {
      let value = self.to_bits();
      return value.push_at_buffer(slf, bit_index, bit_size, manage_overflow);
    }
 }
  
  crate::derive_sizeof!(&u128);
  derive_write!(u128);
  derive_push!(u128);
  derive_pushoptimized!(u128);

 impl PushOptimized for i128{
  fn push_at_buffer(&self, slf:&mut DataB, bit_index:u64, bit_size:&mut Option<u64>, manage_overflow:Option<OverFlow>)->Result<(), AllocErr> {
      let value = (*self) as u128;
      return value.push_at_buffer(slf, bit_index, bit_size, manage_overflow);
  }
 }
 #[derive(Debug, Clone, Copy)]
 pub struct Vars{
  offset:u64,
  bit:u64,
  no_zero_bit:bool,
  byte_overflow:bool,
  t_size:u64,
  size:u64,
  no_bit_size:bool,
  overflow:Option<OverFlowStrategy>,
  overflow_len:Option<u64>,
  remaining_space:u64
 }
  impl Vars{
  pub fn new()->Self{
    Vars{
      offset:0,
      bit:0,
      no_zero_bit:false,
      size:0,
      t_size:0,
      no_bit_size:true,
      overflow:None,
      overflow_len:None,
      byte_overflow:false,
      remaining_space:0,
    }
  }
  pub fn init<T:SizeOfT+?Sized>(&mut self, slf:&mut DataB, bit_index:u64, bit_size:&mut Option<u64>, _value:&T, manage_overflow:Option<OverFlow>)->Result<(), AllocErr>{
    if check_num(bit_index, u64::MAX){return Err(AllocErr::ArithmeticOverflow);}
    let offset_bit = byte_from_bits(bit_index, true)?;
    self.byte_overflow = byte_overflow(offset_bit.1, bit_size)?;
    self.t_size = t_size(_value, bit_size, self.byte_overflow)?;
    if self.t_size <=0{
      let b = bytes_to_bits(offset_bit.0)?;
      if let Some(n) = offset_bit.1{
      if n != 0{move_ptr_val(slf, b-BYTE_LENGTH,n)?;}
      }
      else{move_ptr_val(slf, b-BYTE_LENGTH, 0)?;}
      return Ok(());
    }
    if let Some(n) = bit_size{self.size = *n; self.no_bit_size=false;}
    else{self.size = bytes_to_bits(self.t_size)?; self.no_bit_size=true;}
    self.no_zero_bit = false;
    self.overflow_len = None;
    if let Some(n) = offset_bit.1{
      if n != 0{self.no_zero_bit = true;}
      self.bit = n;
    }
    self.overflow = None;
  
    let mut m_overflow = OverFlow::LazyFail;
    match manage_overflow{
      Some(i) =>{
        m_overflow = i;
      },
      None=>{},
    }
    let mut ovfw_len =0;
    let mut remaining_s = 0;
    self.offset = sub(offset_bit.0,1);
      if let Err(i) = validation_for_push(slf.get_size(), self.offset, offset_bit.1, self.t_size, *bit_size, m_overflow, &mut ovfw_len, &mut remaining_s){
          match i {
            AllocErr::Overflow(n)=>{
              match n {
                OverFlowStrategy::LazyFail(s)=>{
                  return Err(AllocErr::Overflow(OverFlowStrategy::LazyFail(s)));
                },
                OverFlowStrategy::RemainingData(_)=>{
                  self.overflow = Some(n);
                  self.t_size = calc_remaining(slf.get_size() as u64, self.offset);
                  self.overflow_len = Some(ovfw_len);
                }
                _=>{
                  self.overflow = Some(n);
                  self.t_size = calc_remaining(slf.get_size() as u64, self.offset);
                }
              }
            },
            _=>{
              return Err(i);
            }
          }
      }
       self.remaining_space = remaining_s;

      let b = bytes_to_bits(offset_bit.0)?;
      if self.no_zero_bit{
        move_ptr_val(slf, b-BYTE_LENGTH, self.bit)?;
      }
      else{
        move_ptr_val(slf, b-BYTE_LENGTH, 0)?;
      }
    return Ok(());
 }
  pub fn ret_overflow<T:Copy>(overflow:Option<OverFlowStrategy>, for_remaining_data:Option<u64>, data_vec:&[T], index:usize)->Result<(), AllocErr>{
     if let Some(vrflow) = overflow{
        match vrflow{
          OverFlowStrategy::RemainingData(n)=>{
            if let Some(len) = for_remaining_data{
              unsafe{
                let buffer = n as *mut T;
               for i in 0..len as usize{
                 *buffer.add(i) = data_vec[index];
               }
              }
            }else{
              return Err(AllocErr::RemainingDataStrategyErr);
            }
          },
           _=>{}
        }
      return Err(AllocErr::Overflow(vrflow));
    }
    Ok(())
  }
  pub fn fits<T:Copy>(mut_vars:&mut DataB, data:T, data_size:u64, offset:u64)->Result<(),AllocErr>{
    copy_data(mut_vars.get_vec() as *mut T, offset, data);
    move_ptr_val(mut_vars,mut_vars.ptr(), data_size)
 }
  pub fn get_byte_offset(&self)->u64{
    self.offset
  }
  pub fn get_bit_offset(&self)->u64{
    self.bit
  }
  pub fn get_start_in_zero_flag(&self)->bool{
    !self.no_zero_bit
  }
  pub fn get_byte_overflow_flag(&self)->bool{
    self.byte_overflow
  }
  pub fn get_size_of_push(&self)->u64{
    self.t_size
  }
  pub fn get_bit_size_val(&self)->u64{
    self.size
  }
  pub fn get_no_bit_size_flag(&self)->bool{
    self.no_bit_size
  }
  pub fn get_overflow(&self)->Option<OverFlowStrategy>{
    self.overflow
  }
  pub fn get_overflow_len(&self)->Option<u64>{
    self.overflow_len
  }
  pub fn get_remaining_space(&self)->u64{
    self.remaining_space
  }
  pub fn set_byte_offset(&mut self, new:u64){
    self.offset = new;
  }
  pub fn set_bit_offset(&mut self, new:u64){
    self.bit = new;
  }
  pub fn set_start_in_zero_flag(&mut self, new:bool){
    self.no_zero_bit = new;
  }
  pub fn set_byte_overflow_flag(&mut self, new:bool){
    self.byte_overflow = new;
  }
  pub fn set_size_of_push(&mut self, new:u64){
    self.t_size = new;
  }
  pub fn set_bit_size_val(&mut self, new:u64){
    self.size = new;
  }
  pub fn set_no_bit_size_flag(&mut self, new:bool){
    self.no_bit_size = new;
  }
  pub fn set_overflow(&mut self, new:Option<OverFlowStrategy>){
    self.overflow = new;
  }
  pub fn set_overflow_len(&mut self, new:Option<u64>){
    self.overflow_len = new;
  }
  pub fn set_remaining_space(&mut self, new:u64){
    self.remaining_space = new;
  }

 }


 pub fn return_fn<T>(ptr:*mut T, ptr_size:u64, masks:&[T], datas:&[T], index:u64, main_structure:&mut DataB, 
  overflow:Option<OverFlowStrategy>, overflow_len:Option<u64>, for_move_ptr:u64)->Result<(), AllocErr>
  where
  T:Copy +
  BitAnd<Output = T> +
  BitOr<Output =T>{
    let j=put_datas(ptr, ptr_size as usize,&masks, &datas, index as usize)?;
    move_ptr_val(main_structure, main_structure.ptr(), for_move_ptr)?;
    return Vars::ret_overflow(overflow, overflow_len, &datas, j);
 } 
 pub fn sel_write_strategy<T>(ptr:*mut T,ptr_size:u64, offset:u64, t_size:u64, available_space:u64)->Result<(Option<u64>, BytesSlice<T>), AllocErr>{
    if check_num(offset, u64::MAX-ptr_size){return Err(AllocErr::ArithmeticOverflow);}
    else if check_num(offset, ptr_size){return Err(AllocErr::OutOfBounds(ptr_size));}
    let addr = ptr as u64;
    let mut slice = BytesSlice::_8;
    let mut padding = 0;
    match sel_aligning_with_memory(t_size,aligned_with_memory(addr, offset)?){
      Aligned::Bits8=>{return Ok((None, slice));},
      Aligned::Bits16=>{
        slice = BytesSlice::_16;
        if let Some(n) = padding_to_align(t_size, 2){
          padding = n;
        }
      },
      Aligned::Bits32=>{
        slice = BytesSlice::_32;
        if let Some(n) = padding_to_align(t_size, 4){
          padding = n;
        }},
      Aligned::Bits64=>{
        slice = BytesSlice::_64;
        if let Some(n) = padding_to_align(t_size, 8){
          padding = n;
        }
      }
    }
    if padding != 0{
      if available_space >= padding{
        return Ok((Some(sum(t_size,padding)?), slice));
      }
      return Ok((None, BytesSlice::_8));
    }
    return Ok((None, slice));
 }
 pub fn move_ptr_val(ptr:&mut DataB,ptr_value:u64, plus:u64)->Result<(), AllocErr>{
  ptr.move_ptr(sum(ptr_value,plus)?);
  Ok(())
 }
 pub fn copy_data<T:Copy>(ptr:*mut T, offset:u64, data:T){
  unsafe{*ptr.add(offset as usize) = data;}
 }
 pub fn put_datas<T>(ptr:*mut T, ptr_size:usize, masks:&[T], values:&[T], index:usize)->Result<usize, AllocErr>
 where T:BitAnd<Output=T> +
 BitOr<Output = T>+
 Copy
 {
  let mut j = index;
  let check = sum(index as u64, values.len() as u64)?;
  if check > ptr_size as u64{return Err(AllocErr::OutOfBounds(ptr_size as u64));}
  for i in 0..ptr_size{
    unsafe{*ptr.add(i) = or(and(*(ptr.add(i)), masks[j]), values[j]);}
    j+=1;
  }
  Ok(j)
 }


 pub fn t_size<T:SizeOfT + ?Sized>(_value:&T, bit_size:&mut Option<u64>, byte_overflow:bool)->Result<u64, AllocErr>{
    let mut t_size = size_of(_value) as u64; // gets the size of the value
    if let Some(b) = bit_size{
      if *b <= 0{return Ok(0);}
      if *b > bytes_to_bits(t_size)?{*b = bytes_to_bits(t_size)?;} //fixed the bit_sized variable, because it cannot be greather than the len of the data
      else{t_size = sub(bits_to_bytes(*b)?,1);} 
      
    }
    //add 1 byte if the bit where start to insert the value is greather than 0
    if byte_overflow{
      t_size = t_size.wrapping_add(1);
    }
    Ok(t_size)
 }
 pub fn byte_overflow(bit:Option<u64>, bit_size:&mut Option<u64>)->Result<bool, AllocErr>{
  if let Some(i) = bit{
    if let Some(j) = bit_size{
      if sum(*j,i)?<bytes_to_bits(bits_to_bytes(*j)?)?{return Ok(false);}
      return Ok(true);
    }else{return Ok(true);}
  }else{return Ok(false);}
 }
 pub fn validation_for_push(block_size:usize,byte:u64, bit:Option<u64>, t_size:u64, bit_size:Option<u64>, overflow_f:OverFlow, overflow_len:&mut u64, remaining_space:&mut u64)-> Result<(),AllocErr>{
  // check wether the byte offset is greather than the size of the block data
  if byte > block_size as u64{
        return Err(AllocErr::OutOfBounds(block_size as u64));
  }
  // Check wether does exist overflow
  if let Some(b) = bit_size{
   if let Some(i) = bit{
    let remaining_bits = calc_remaining(calc_remaining(bytes_to_bits(block_size as u64)?, bytes_to_bits(byte)?), i);
     return manage_overflow(overflow_f, remaining_bits, b, false, overflow_len, remaining_space);
   }
    let t_size_b = bits_to_bytes(b)?;
    let remaining_bytes = calc_remaining(block_size as u64, byte);
   return manage_overflow(overflow_f, remaining_bytes, t_size_b, true, overflow_len, remaining_space);  
  }
  if let Some(i) = bit{
    let b = bytes_to_bits(t_size as u64)?;
    let remaining_bits = calc_remaining(calc_remaining(bytes_to_bits(block_size as u64)?, bytes_to_bits(byte)?), i);
    return manage_overflow(overflow_f, remaining_bits, b, false, overflow_len, remaining_space);
   }
    let remaining_bytes = calc_remaining(block_size as u64, byte);
    manage_overflow(overflow_f, remaining_bytes, t_size, true, overflow_len, remaining_space)
 }
 fn calc_remaining(first:u64, second:u64)->u64{
  sub(first,second)
 }
 fn check_remaining(remaining:u64, t_size:u64)->bool{
   t_size > remaining
 }
 fn overflow_strategy(overflow:OverFlow, remaining:u64, t_size:u64, as_byte:bool, overflow_len:&mut u64, remaining_space:&mut u64)->Result<OverFlowStrategy, AllocErr>{
   match overflow{
    OverFlow::LazyFail=>{
      if !as_byte{ return Ok(OverFlowStrategy::LazyFail(remaining));}
      else{ *remaining_space = remaining; return Ok(OverFlowStrategy::LazyFail(bytes_to_bits(remaining)?));}
    }
    OverFlow::BitStart=>{
      if !as_byte{ return Ok(OverFlowStrategy::BitStart(remaining));}
      else{ *remaining_space = remaining; return Ok(OverFlowStrategy::BitStart(bytes_to_bits(remaining)?));}
    }
    OverFlow::RetRemainingData=>{
      let mut _overflow_sz = 0;
          if as_byte{_overflow_sz = calc_remaining(t_size, remaining);}
          else{_overflow_sz = bits_to_bytes(calc_remaining(t_size, remaining))?;}
          match new_alloc(_overflow_sz as usize){
            Some(i)=>{
              *overflow_len = _overflow_sz;
              return Ok(OverFlowStrategy::RemainingData(i));
            }
            None =>{
              if !as_byte{ return Ok(OverFlowStrategy::Alternative(remaining));}
              else{ return Ok(OverFlowStrategy::Alternative(bytes_to_bits(remaining)?));}
          }
    }
   }
  }
 }
 fn manage_overflow(overflow:OverFlow, remaining:u64, t_size:u64, as_byte:bool, overflow_len:&mut u64, remaining_space:&mut u64)->Result<(),AllocErr>{
    if check_remaining(remaining, t_size){
      return Err(AllocErr::Overflow(overflow_strategy(overflow, remaining, t_size, as_byte, overflow_len, remaining_space)?));
    }
    Ok(())
 }
 
 fn new_alloc(size:usize)->Option<*mut u8>{
  unsafe{
            let layout = Layout::from_size_align(size, 8).unwrap();
            let ptr = alloc(layout);
            if  ptr.is_null(){
              return None;
            }
            return Some(ptr);
          }
 }
