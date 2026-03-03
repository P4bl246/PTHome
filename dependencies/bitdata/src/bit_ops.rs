 use crate::ops::*;
 use crate::enums::AllocErr;
 use crate::traits::{SecureSub, SecureShift, Or, And};

 const BYTE_LENGTH:usize = 8;
 #[inline(always)]
 pub fn merge<T>(ptr:*mut T, offset:usize, data:T, mask:T, shift:T, byte_len:T)->T
 where T: Copy + SecureSub<Output=T> +
 SecureShift<Output=T>+
 And<Output = T>+
 Or<Output = T>
 {
  unsafe{or(bit_filter(*ptr.add(offset), mask, shift, true), bit_filter(data, mask, sub(byte_len,shift), false))}
 }
 #[inline(always)]
 pub fn bit_filter<T>(bitwise_v:T, v_to_shift:T, v2_to_shift:T, shift_to_right:bool)->T
 where 
 T:SecureShift<Output=T>+
  And<Output = T>
 {
    return and(bitwise_v, shift(v_to_shift, v2_to_shift, shift_to_right)); 
  }
 #[inline(always)]
 pub fn continue_bit<T>(value_1:T, start_in_bit:T, continues_in:T, byte_len:T, is_big_endian:bool)->T
 where 
 T:SecureShift<Output=T> + 
 SecureSub<Output=T> + 
 Or<Output=T> + Copy
  {
    or(shift(value_1, start_in_bit, !is_big_endian),shift(continues_in, sub(byte_len,start_in_bit), is_big_endian))
 }
 //----------------------------------------------------------
 /// # `byte_from_bits`
 /// Calculates the byte index and optionally the bit index within that byte for a given bit index, ensuring that the calculations do not result in an arithmetic overflow.
 /// 
 /// # Arguments
 /// * `bit` - The bit index for which to calculate the byte and bit indices.
 /// * `bit_byte` - A boolean indicating whether to calculate the bit index within the byte (true) or not (false).
 /// 
 /// # Example
 /// ```rust
 /// use bitdata::ops::byte_from_bits;
 /// let result = byte_from_bits(10, true);
 /// assert_eq!(result, Ok((2, Some(2))));
 /// let result = byte_from_bits(10, false);
 /// assert_eq!(result, Ok((2, None)));
 /// ```
 /// 
 /// # Returns
 /// * `Ok((usize, Option<usize>))` - A tuple containing the byte index and an optional bit index within that byte if the calculations are successful.
 /// * `Err(AllocErr)` - An error if the calculations result in an arithmetic overflow.
 #[inline]
 pub fn byte_from_bits(bit:usize, bit_byte:bool) -> Result<(usize, Option<usize>), AllocErr>{
     let byte = bits_to_bytes(bit)?; // gets the byte where the bit is allocated
     if bit_byte{
     let mut bit_i = bit;
     if bit_i >= BYTE_LENGTH{bit_i = bit - (bytes_to_bits(byte)?-BYTE_LENGTH); }// gets the number of the bit where start to push the value inside the byte
      return Ok((byte, Some(bit_i)));
     }
     Ok((byte, None))
 }

 /// # `bits_to_bytes`
 /// Converts a given number of bits to bytes, ensuring that the conversion does not result in an arithmetic overflow.
 /// The function takes a `bit_size` as input and checks if it exceeds the maximum allowed value for conversion. If it does,
 /// it returns an error; otherwise, it performs the conversion by dividing the total number of bits 
 /// plus 8 (to account for any partial byte), by the number of bits in a byte.
 /// 
 /// # Arguments
 /// * `bit_size` - The number of bits to be converted to bytes.
 /// 
 /// # Example
 /// ```rust
 /// use bitdata::ops::bits_to_bytes;
 /// let result = bits_to_bytes(16);
 /// assert_eq!(result, Ok(3));
 /// let result = bits_to_bytes(0xFFFFFFFFFFFFFFF7);
 /// assert_eq!(result, Err(AllocErr::ArithmeticOverflow));
 /// let result = bits_to_bytes(0);
 /// assert_eq!(result, Ok(1));
 /// ```
 /// 
 /// # Returns
 /// * `Ok(usize)` - The number of bytes corresponding to the given number of bits if the conversion is successful.
 /// * `Err(AllocErr)` - An error indicating an arithmetic overflow if the input exceeds the maximum allowed value for conversion.
 #[inline(always)]
 pub fn bits_to_bytes(bit_size:usize)->Result<usize, AllocErr>{
  Ok(div(sum(bit_size,BYTE_LENGTH)?, BYTE_LENGTH)?)
 }
 //----------------------------------------------------------
 /// # `bytes_to_bits`
 /// Converts a given number of bytes to bits, ensuring that the conversion does not result in an arithmetic overflow.
 /// The function takes a `byte_size` as input and checks if it exceeds the maximum allowed value for conversion. If it does,
 /// it returns an error; otherwise, it performs the conversion by multiplying the number of bytes by the number of bits in a byte.
 /// 
 /// # Arguments 
 /// * `byte_size` - The number of bytes to be converted to bits.
 ///
 /// # Example
 /// ```rust
 /// use bitdata::ops::bytes_to_bits;
 /// let result = bytes_to_bits(2);
 /// assert_eq!(result, Ok(16));
 /// let result = bytes_to_bits(0x1FFFFFFFFFFFFFFF);
 /// assert_eq!(result, Err(AllocErr::ArithmeticOverflow));
 /// let result = bytes_to_bits(0);
 /// assert_eq!(result, Ok(0));
 /// ```
 /// 
 /// # Returns
 /// * `Ok(usize)` - The number of bits corresponding to the given number of bytes if the conversion is successful.
 /// * `Err(AllocErr)` - An error indicating an arithmetic overflow if the input exceeds the maximum allowed value for conversion.
 #[inline(always)]
 pub fn bytes_to_bits(byte_size:usize)->Result<usize, AllocErr>{
   if check_num(byte_size, 0x1FFFFFFFFFFFFFFF){return Err(AllocErr::ArithmeticOverflow);};
  Ok(mult(byte_size, BYTE_LENGTH)?)
 }
 