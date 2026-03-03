 #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
 /// This enum defines the strategy to handle overflow situations during memory allocation or data manipulation.
 /// 
 /// - `RetRemainingData`: This strategy indicates that in case of an overflow, the remaining data should be returned. 
 /// This can be useful when you want to process as much data as possible without failing completely.
 /// 
 /// - `BitStart`: This strategy indicates that in case of an overflow, the operation will process
 ///  all before the overflow bit and returns the starting bit of the overflow. This can be useful when you want to know where 
 /// the overflow occurred and potentially handle it and avoid a non-predictable
 /// performance behavior due the allocation of memory as in `RemainingData`.
 /// 
 /// - `LazyFail`: This strategy indicates that in case of an overflow, the operation should fail lazily. This means 
 /// that if does exist overflow the operation will return an error, but if the overflow is not detected, it will proceed as normal. 
 /// This can be useful when you don't want to slice your data in differents buffers.
 /// 
 /// The default strategy is `LazyFail`, which means that if an overflow occurs, the operation will fail lazily, 
 /// returning an error without slicing the data.
 #[repr(C)]
 pub enum OverFlow{
  RetRemainingData,
  BitStart,
  #[default]
  LazyFail
}
 
 /// This enum defines the strategy to handle overflow situations during memory allocation or data manipulation.
 /// 
 /// - `RemainingData`: This strategy indicates that in case of an overflow, the remaining data
 /// should be returned. This can be useful when you want to process as much data as possible without failing completely.
 /// - `Alternative`: This strategy indicates that in case of an overflow, an alternative value should be returned. 
 /// This can be useful when you want to provide a fallback value in case of an overflow.
 /// 
 /// - `BitStart`: This strategy indicates that in case of an overflow, 
 /// the operation will process all before the overflow bit and returns the starting bit of the overflow. 
 /// This can be useful when you want to know where the overflow occurred and potentially handle it and avoid a 
 /// non-predictable performance behavior due the allocation of memory as in `RemainingData`.
 /// 
 /// - `LazyFail`: This strategy indicates that in case of an overflow, the operation should fail lazily. 
 /// This means that if does exist overflow the operation will return an error, but if the overflow is not 
 /// detected, it will proceed as normal. This can be useful when you don't want to slice your data in differents buffers.
 /// 
 /// - `Alternative`: This strategy indicates that in case of an overflow, and the RemainingData strategy was 
 /// select but can't allocate memory, the operation will return an alternative value. 
 /// This can be useful when you want to provide a fallback value in case of an overflow and the RemainingData 
 /// strategy was select but can't allocate memory.
 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
 #[repr(C)]
pub enum OverFlowStrategy{
  RemainingData(*mut u8),
  Alternative(usize),
  BitStart(usize),
  LazyFail(usize)
}
 
 /// This enum defines the possible errors that can occur during memory allocation or data manipulation.
 /// 
 /// - `AllocationFailed`: This error indicates that the memory allocation failed. 
 /// This can occur when there is not enough memory available to allocate.
 /// 
 /// - `Overflow`: This error indicates that an overflow occurred during memory allocation or data manipulation. 
 /// The `Overflow` variant contains an `OverFlowStrategy` which provides information about how to handle the overflow situation.
 /// 
 /// - `OutOfBounds`: This error indicates that an out-of-bounds access occurred. 
 /// The `OutOfBounds` variant contains a `usize` value which represents the out-of-bounds address or index that was accessed.
 ///
 /// - `ArithmeticOverflow`: This error indicates that an arithmetic overflow occurred if the calculation could generate an arithmetic overflow . 
 /// This can occur when the result of a calculation exceeds the maximum value that can be represented by the data type being used.
 /// 
 /// - `RemainingDataStrategyErr`: This error indicates that there was an error with the RemainingData strategy. 
 /// This can occur when the RemainingData strategy was selected but there was an issue with allocating memory
 /// 
 /// - `UnrecognizedInstruction`: This error indicates that an unrecognized instruction was encountered. 
 /// This can occur when the operation being performed is not recognized or supported.
 /// 
 /// The `AllocErr` enum provides a way to categorize and handle different types of errors that can occur during memory allocation or data manipulation, allowing for more robust error handling in the code.
 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
 #[repr(C)]
pub enum AllocErr{
    AllocationFailed,
    Overflow(OverFlowStrategy),
    OutOfBounds(usize),
    ArithmeticOverflow,
    RemainingDataStrategyErr,
    UnrecognizedInstruction
}
 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
 #[repr(C)]
pub enum BytesSlice<T>{
  _8,
  _16,
  _32,
  _64,
  _N(T)
}
 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
 #[repr(C)]

 /// This enum defines a set of typed pointers for different data types. Each variant of the `TypedPtr` enum
 ///  represents a pointer to a specific data type, allowing for type-safe handling of pointers in Rust.
 /// 
 /// - `U8(*mut u8)`: This variant represents a pointer to an unsigned 8-bit integer (u8). It allows for mutable 
 /// access to a u8 value through the pointer.
 /// 
 /// - `U16(*mut u16)`: This variant represents a pointer to an unsigned 16-bit integer (u16). It allows for mutable 
 /// access to a u16 value through the pointer.
 /// 
 /// - `U32(*mut u32)`: This variant represents a pointer to an unsigned 32-bit integer (u32). It allows for mutable
 /// access to a u32 value through the pointer.
 /// 
 /// - `U64(*mut u64)`: This variant represents a pointer to an unsigned 64-bit integer (u64). It allows for mutable
 /// access to a u64 value through the pointer.
pub enum TypedPtr<T> {
    U8(*mut u8), 
    U16(*mut u16),
    U32(*mut u32), 
    U64(*mut u64),
    _N(*mut T)
}
 /// This enum defines a set of data types that can be used in the context of memory manipulation or data processing. 
 /// 
 /// Each variant of the `DataType` enum represents a specific unsigned integer type, allowing for type-safe handling of data in Rust.
 /// 
 /// - `U8(u8)`: This variant represents an unsigned 8-bit integer (u8). It can hold values from 0 to 255.
 /// 
 /// - `U16(u16)`: This variant represents an unsigned 16-bit integer (u16). It can hold values from 0 to 65,535.
 /// 
 /// - `U32(u32)`: This variant represents an unsigned 32-bit integer (u32). It can hold values from 0 to 4,294,967,295.
 /// 
 /// - `U64(u64)`: This variant represents an unsigned 64-bit integer (u64). It can hold values from 0 to 18,446,744,073,709,551,615.
 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
 #[repr(C)]
pub enum DataType<T>{
  U8(u8),
  U16(u16),
  U32(u32),
  U64(u64),
  _N(T)
}

 /// This enum defines a set of  alignment options for different data types. Each variant of the `MemAligned` enum
 /// represents a specific alignment requirement, allowing for type-safe handling of alignment in Rust.
 /// 
 /// - `Bits64`: This variant represents a  alignment requirement of 64 bits (8 bytes). 
 /// It indicates that the data should be aligned to a 64-bit boundary .
 /// 
 /// - `Bits32`: This variant represents a  alignment requirement of 32 bits (4 bytes).
 /// It indicates that the data should be aligned to a 32-bit boundary.
 /// 
 /// - `Bits16`: This variant represents a  alignment requirement of 16 bits (2 bytes).
 /// It indicates that the data should be aligned to a 16-bit boundary.
 /// 
 /// - `Bits8`: This variant represents a  alignment requirement of 8 bits (1 byte).
 /// It indicates that the data should be aligned to an 8-bit boundary.
 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
 #[repr(C)]
 pub enum Aligned{
  Bits64,
  Bits32,
  Bits16,
  Bits8
}
