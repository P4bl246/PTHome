 use std::ops::{BitAnd, BitOr};
 use crate::enums::AllocErr;
 const BYTE_LENGTH:u64 = 8;
 use crate::align_ops::multiple_of;
 /// # `mult`
 /// Multiplies two numbers, if either of them is a multiple of 2, it will use bit shifting to perform the multiplication, otherwise it will return an error.
 /// 
 /// # Arguments
 /// * `factor_1` - The first factor to be multiplied.
 /// * `factor_2` - The second factor to be multiplied.
 /// 
 /// # Example
 /// ```rust
 /// use bitdata::ops::mult;
 /// let result = mult(4, 5);
 /// assert_eq!(result, Ok(20));
 /// let result = mult(3, 5);
 /// assert_eq!(result, Err(AllocErr::UnrecognizedInstruction));
 /// ```
 /// 
 /// # Returns
 /// * `Ok(u64)` - The result of the multiplication if successful.
 /// * `Err(AllocErr)` - An error if neither factor is a multiple of 2.
 #[inline(always)]
 pub fn mult(factor_1:u64, factor_2:u64)->Result<u64, AllocErr>{
    if multiple_of(factor_2, 1) == 0{return Ok(shift(factor_1, factor_2.trailing_zeros() as u64, false));}
    if multiple_of(factor_1, 1) == 0 {return Ok(shift(factor_2, factor_1.trailing_zeros() as u64, false));}
     Err(AllocErr::UnrecognizedInstruction)
 }
 //-----------------------------------------------------------
 /// # `div`
 /// Divides two numbers, if the divisor is a multiple of 2, it will use bit shifting to perform the division, 
 /// otherwise it will return an error.
 ///
 /// # Arguments
 /// * `dividend` - The number to be divided.
 /// * `divisor` - The number to divide by.
 /// 
 /// # Example
 /// ```rust
 /// use bitdata::ops::div;
 /// let result = div(20, 4);
 /// assert_eq!(result, Ok(5));
 /// let result = div(20, 3);
 /// assert_eq!(result, Err(AllocErr::UnrecognizedInstruction));
 /// ```
 /// 
 /// # Returns
 /// * `Ok(u64)` - The result of the division if successful.
 /// * `Err(AllocErr)` - An error if the divisor is not a multiple of 2.
 #[inline(always)]
 pub fn div(dividend:u64, divisor:u64)->Result<u64, AllocErr>{
    if multiple_of(divisor, 1) != 0{return  Err(AllocErr::UnrecognizedInstruction);}
     Ok(shift(dividend, divisor.trailing_zeros() as u64, true))
 }
 
 //----------------------------------------------------------
 /// # `and`
 /// Performs a bitwise AND operation between two values of the same type.
 /// 
 /// # Arguments
 /// * `left` - The first value for the AND operation.
 /// * `right` - The second value for the AND operation.
 /// 
 /// # Example
 /// ```rust
 /// use bitdata::ops::and;
 /// let result = and(5, 3);
 /// assert_eq!(result, 1);
 /// let result = and(5, 2);
 /// assert_eq!(result, 0);
 /// ```
 /// 
 /// # Returns
 /// * `T` - The result of the bitwise AND operation between `left` and `right`.
 #[inline(always)]
 pub fn and<T>(left:T, right:T)->T
 where T:BitAnd<Output = T>{
  left&right
 }
 //----------------------------------------------------------
 /// # `or`
 /// Performs a bitwise OR operation between two values of the same type.
 /// 
 /// # Arguments
 /// * `left` - The first value for the OR operation.
 /// * `right` - The second value for the OR operation.
 /// 
 /// # Example
 /// ```rust
 /// use bitdata::ops::or;
 /// let result = or(5, 3);
 /// assert_eq!(result, 7);
 /// let result = or(5, 2);
 /// assert_eq!(result, 7);
 /// ```
 /// 
 /// # Returns
 /// * `T` - The result of the bitwise OR operation between `left` and `right`.
 #[inline(always)]
 pub fn or<T>(left:T, right:T)->T
 where T: BitOr<Output = T>{
  left|right
 }
 //----------------------------------------------------------

  /// # `SecureShift`
  /// A trait that provides a secure way to perform bit shifting operations on various integer types.
  /// The `secure_sh` method takes a shift amount and a boolean indicating the direction of the shift (right or left) 
  /// and returns the result of the shift operation. If the shift amount is greater than or equal to the number of 
  /// bits in the type, it returns 0 to prevent undefined behavior.
  pub trait SecureShift{
    type Output;
    fn secure_sh(&self, shift: Self::Output, to_right:bool)->Self::Output;
}
macro_rules! impl_secureshift{
    ($($ty:ty),*) => {
        $(
            impl SecureShift for $ty {    
                type Output = $ty;
                fn secure_sh(&self, shift: $ty, to_right:bool)->$ty{
                    let s = (std::mem::size_of::<$ty>() << 3) as $ty;
                    if  shift >= s{return 0 as $ty;}
                    if to_right{return  *self>>shift;}
                    return *self<<shift;
                }
            }
        )*
    };
}

impl_secureshift!(
    u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, u128, i128
);
 /// # `shift`
 /// Performs a secure bit shift operation on a value of type `T` by a specified number of bits in either direction (left or right).
 /// The function takes three parameters: the value to be shifted, the number of bits to shift, and a boolean indicating 
 /// the direction of the shift (true for right, false for left).
 /// The function uses the `SecureShift` trait to ensure that the shift operation is performed securely, preventing
 /// undefined behavior when the shift amount exceeds the bit width of the type.
 /// 
 /// # Arguments
 /// * `value` - The value to be shifted.
 /// * `shift` - The number of bits to shift.
 /// * `to_right` - A boolean indicating the direction of the shift (true for right, false for left).
 /// 
 /// # Example
 /// ```rust
 /// use bitdata::ops::shift;
 /// let result = shift(16u64, 2, true);
 /// assert_eq!(result, 4);
 /// let result = shift(16u64, 2, false);
 /// assert_eq!(result, 64);
 /// let result = shift(16u64, 64, true);
 /// assert_eq!(result, 0);
 /// ```
 /// 
 /// # Returns
 /// * `T` - The result of the secure bit shift operation on the input value.
 #[inline(always)]
 pub fn shift<T>(value:T, shift:T, to_right:bool)->T
 where
  T:SecureShift<Output=T>
 {
  value.secure_sh(shift, to_right)
 }
 //----------------------------------------------------------
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
 /// * `Ok(u64)` - The number of bytes corresponding to the given number of bits if the conversion is successful.
 /// * `Err(AllocErr)` - An error indicating an arithmetic overflow if the input exceeds the maximum allowed value for conversion.
 #[inline(always)]
 pub fn bits_to_bytes(bit_size:u64)->Result<u64, AllocErr>{
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
 /// * `Ok(u64)` - The number of bits corresponding to the given number of bytes if the conversion is successful.
 /// * `Err(AllocErr)` - An error indicating an arithmetic overflow if the input exceeds the maximum allowed value for conversion.
 #[inline(always)]
 pub fn bytes_to_bits(byte_size:u64)->Result<u64, AllocErr>{
   if check_num(byte_size, 0x1FFFFFFFFFFFFFFF){return Err(AllocErr::ArithmeticOverflow);};
  Ok(mult(byte_size, BYTE_LENGTH)?)
 }
 //----------------------------------------------------------
 /// # `check_num`
 /// Checks if a given number is greater than or equal to a specified maximum value, returning a boolean result.
 /// The function takes two parameters: `num`, the number to be checked, and `max`, the maximum value for comparison.
 /// If `num` is greater than or equal to `max`, the function returns `true`; otherwise, it returns `false`.
 /// This function is used to prevent arithmetic overflow in the conversion functions.
 /// 
 /// # Arguments
 /// * `num` - The number to be checked against the maximum value.
 /// * `max` - The maximum value for comparison.
 /// 
 /// # Example
 /// ```rust
 /// use bitdata::ops::check_num;
 /// let result = check_num(10, 5);
 /// assert_eq!(result, true);
 /// let result = check_num(5, 10);
 /// assert_eq!(result, false);
 /// let result = check_num(10, 10);
 /// assert_eq!(result, true);
 /// ```
 /// 
 /// # Returns
 /// * `bool` - `true` if `num` is greater than or equal to `max`, otherwise `false`.
 #[inline(always)]
 pub fn check_num<T:PartialOrd>(num:T, max:T)->bool{
  num >=max
 }
 //----------------------------------------------------------

   /// # `SecureSub`
  pub trait SecureSub{
    type Output;
    fn secure_sub(&self, sustrahend: Self::Output)->Self::Output;
}
macro_rules! impl_securesubsigned{
    ($($ty:ty),*) => {
        $(
            impl SecureSub for $ty {    
                type Output = $ty;
                fn secure_sub(&self, sustrahend: $ty)->$ty{
                    if  sustrahend >= *self{return 0 as $ty;}
                    return *self-sustrahend;
                }
            }
        )*
    };
}
impl_securesubsigned!(
    i8, i16, i32, i64, isize, i128, f32, f64
);


macro_rules! impl_securesubunsigned{
    ($($ty:ty),*) => {
        $(
            impl SecureSub for $ty {    
                type Output = $ty;
                fn secure_sub(&self, sustrahend: $ty)->$ty{
                    return (*self).saturating_sub(sustrahend);
                }
            }
        )*
    };
}

impl_securesubunsigned!(
    u8, u16, u32, u64, usize, u128
);
 /// # `sub`
 /// Performs a subtraction operation between two unsigned 64-bit integers, ensuring that the result does not underflow.
 /// The function takes two parameters: `minuend`, the number from which another number (the subtrahend) is to be subtracted, 
 /// and `subtrahend`, the number to be subtracted from the minuend.
 /// If the subtrahend is greater than or equal to the minuend, the function returns 0 to prevent underflow; otherwise, 
 /// it returns the result of the subtraction.
 /// 
 /// # Arguments
 /// * `minuend` - The number from which another number is to be subtracted.
 /// * `subtrahend` - The number to be subtracted from the minuend.
 /// 
 /// # Example
 /// ```rust
 /// use bitdata::ops::sub;
 /// let result = sub(10, 5);
 /// assert_eq!(result, 5);
 /// let result = sub(5, 10);
 /// assert_eq!(result, 0);
 /// ```
 /// 
 /// # Returns
 /// * `u64` - The result of the subtraction if no underflow occurs, otherwise 0.
 #[inline(always)]
 pub fn sub<T:SecureSub<Output=T>>(minuend:T, subtrahend:T)->T{
    minuend.secure_sub(subtrahend)
 }
//----------------------------------------------------------
 pub trait SecureSume{
    type Output;
    fn secure_sum(&self, addend: Self::Output)->Result<Self::Output, AllocErr>;
 }

macro_rules! impl_securesume{
    ($($ty:ty),*) => {
        $(
            impl SecureSume for $ty {    
                type Output = $ty;
                fn secure_sum(&self, addend: $ty)->Result<$ty, AllocErr>{
                    if check_num(*self, <$ty>::MAX - addend){return Err(AllocErr::ArithmeticOverflow);}
                     Ok((*self)+addend)
                }
            }
        )*
    };
 }
impl_securesume!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64, u128, i128);
 /// # `sum`
 /// Performs a secure addition operation between two values of the same type, ensuring that the result does not overflow.
 /// 
 /// # Arguments 
 /// * `augend` - The first value to be added.
 /// * `addend` - The second value to be added.
 /// 
 /// # Example
 /// ```rust
 /// use bitdata::ops::sum;
 /// let result = sum(5u64, 10u64);
 /// assert_eq!(result, Ok(15u64));
 /// let result = sum(u64::MAX, 1u64);
 /// assert_eq!(result, Err(AllocErr::ArithmeticOverflow));
 /// ```
 /// 
 /// # Returns
 /// * `Ok(T)` - The result of the addition if successful.
 /// * `Err(AllocErr)` - An error if the addition results in an overflow.
 #[inline(always)]
 pub fn sum<T>(augend:T, addend:T)->Result<T, AllocErr>
 where T: SecureSume<Output=T>{
    augend.secure_sum(addend)
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
 /// * `Ok((u64, Option<u64>))` - A tuple containing the byte index and an optional bit index within that byte if the calculations are successful.
 /// * `Err(AllocErr)` - An error if the calculations result in an arithmetic overflow.
pub fn byte_from_bits(bit:u64, bit_byte:bool) -> Result<(u64, Option<u64>), AllocErr>{
     let byte = bits_to_bytes(bit)?; // gets the byte where the bit is allocated
     if bit_byte{
     let mut bit_i = bit;
     if bit_i >= BYTE_LENGTH{bit_i = bit - (bytes_to_bits(byte)?-BYTE_LENGTH); }// gets the number of the bit where start to push the value inside the byte
      return Ok((byte, Some(bit_i)));
     }
     Ok((byte, None))
 }
//----------------------------------------------------------
 pub trait SizeOfT{
    fn size_of_t(&self)->usize;
 }

macro_rules! impl_sizeof {
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

impl_sizeof!(u8, u16, u32, u64, usize, u128, i8, i16, i32, i64, isize, i128, f32, f64, bool, char, ());

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

pub fn size_of<T: SizeOfT + ?Sized>(value: &T) -> usize {
    value.size_of_t()
}

