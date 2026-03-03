#![allow(dead_code)]
#![allow(unused_imports)]
#![warn(private_bounds)]
#![allow(arithmetic_overflow)]
use std::alloc::{alloc, dealloc, Layout};
use std::fmt::Alignment;
use std::mem::MaybeUninit;
use crate::enums::*;
use crate::traits::*;
use crate::ops::*;
use crate::bit_ops::*;
use crate::push_ops::*;
use crate::align_ops::*;
use crate::raw_ptr_ops::*;
pub const BYTE_LENGTH:usize = 8;

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


  trait ReadOptimized{
    fn read_from_buffer<T>(&self, bit_start_index:usize, bit_len:usize, into:&mut [T],manage_overflow:bool)->Result<(&[T], usize), AllocErr>;
 }

 impl ReadOptimized for DataB{
    fn read_from_buffer<T>(&self,bit_start_index:usize, bit_len:usize, into:&mut [T], manage_overflow:bool)->Result<(&[T], usize), AllocErr>{
      let vars = Vars::new(bit_len);
      if let Err(e) = vars.init(self, bit_start_index, into, manage_overflow){
        return Err(e);
      }
     let read = 
      Ok((result, padding));
    }
 }

 struct Vars{
  offset:usize,
  bit:usize,
  no_zero_bit:bool,
  len:usize,
  len_bytes:usize,
  array_len:usize
 }
 impl Vars{
  fn new(bit_len:usize)->Self{
    Vars{
      offset:0,
      bit:0,
      len:bit_len,
      no_zero_bit:false,
      len_bytes:0,
      array_len:0
    }
  }
  fn init<T>(&mut self, slf:&DataB, bit_start_index:usize, into:&mut [T], manage_overflow:bool)->Result<(), AllocErr>{
    let f=byte_from_bits(bit_start_index, true)?;
    self.offset = sub(f.0,1);
    match f.1.unwrap(){
      0=>{},
      _=>{self.bit = f.1.unwrap(); self.no_zero_bit = true;}
    }
    if let Err(e) = manage_read_overflow(slf.get_size(),self.offset, self.bit, self.len){
      match e{
        AllocErr::OutOfBounds(n)=>{
          return Err(AllocErr::OutOfBounds(n));
        },
        AllocErr::Overflow(s)=>{
          if manage_overflow{
            match s{
              OverFlowStrategy::LazyFail(n)=>{
                self.len = n;
                self.len_bytes = bits_to_bytes(self.len)?;
                self.array_len = bits_to_bytes(self.len)?;
                if self.array_len > into.len() {
                  self.array_len = into.len();
                }
                return Ok(());
              },
              _=>{return Err(AllocErr::UnrecognizedInstruction);}
          }
        }
        return Err(AllocErr::Overflow(s));
      }
      _=>{return Err(AllocErr::UnrecognizedInstruction);}
      }
    }
    self.len_bytes = bits_to_bytes(sum(self.len, self.bit)?)?;
    self.array_len = bits_to_bytes(self.len)?;
    if self.array_len > into.len() {
      self.array_len = into.len();
    }
    Ok(())
  }
 }
 fn manage_read_overflow(buffer_len:usize, offset:usize, bit:usize, len:usize)->Result<(), AllocErr>{
  if offset > buffer_len{
    return Err(AllocErr::OutOfBounds(buffer_len));
  }
   let n = bytes_to_bits(offset)?;
  let size = bytes_to_bits(buffer_len)?;
  let n2 = sum(n, bit)?;
  if sum(n2,len)? > size{
    return Err(AllocErr::Overflow(OverFlowStrategy::LazyFail(sub(n2, size))));
  }
  Ok(())
 }

 fn sel_read_strategy<T>(ptr:*mut T,ptr_size:usize, offset:usize, len:usize)->Result<BytesSlice::<u8>, AllocErr>{
  if check_num(offset, usize::MAX-ptr_size){return Err(AllocErr::ArithmeticOverflow);}
    else if check_num(offset, ptr_size){return Err(AllocErr::OutOfBounds(ptr_size));}
    let addr = ptr as usize;
    match sel_aligning_with_memory(len,aligned_with_memory(addr, offset)?){
      Aligned::Bits8=>Ok(BytesSlice::_8),
      Aligned::Bits16=>Ok(BytesSlice::_16),
      Aligned::Bits32=>Ok(BytesSlice::_32),
      Aligned::Bits64=>Ok(BytesSlice::_64)
    }
  }

/// # `DataB`
/// A structure for store data at bit level.
/// 
/// # Fields
/// * `ptr` - A mutable pointer to an auxiliary pointer block that enables to indicate as an index the next bit free to use as a pointer of a stack (the vec).
/// * `vec` - A mutable pointer to the data block.
/// * `size` - The size of the data block in bytes.
#[repr(C)]
#[derive(Default, Clone, Debug, PartialEq, Eq)]

pub struct DataB{
    ptr: usize,
    vec: *mut u8,
    size: usize,
}
impl DataB{
    /// # `new`
    /// Creates a new DataB instance with the specified size.
    /// 
    /// # Arguments
    /// * `size` - The size of the data block in bytes.
    /// 
    /// # Example
    /// ```rust
    /// use PTHome::main_code::data_structures::DataB;
    /// let db = DataB::new(1024).unwrap_or(assertion_failed!());
    /// assert!(!db.vec.is_null());
    /// ```
    pub fn new(size:usize)->Result<Self, AllocErr>{
      match new_alloc(size){
        Some(vec_ptr)=>{
          let s = DataB{
             ptr: 0,
             vec:vec_ptr,
             size:size,
           };
          Ok(s)

        },
        None=>{Err(AllocErr::AllocationFailed)},
      }
    }

    /// # `deallocate`
    /// Deallocates the memory allocated for the data block.
    /// 
    /// # Example
    /// ```rust
    /// use PTHome::main_code::data_structures::DataB;
    /// let mut db = DataB::new(1024).unwrap_or(assertion_failed!());
    /// db.deallocate();
    /// assert!(db.vec.is_null() == true);
    /// ```
    pub fn deallocate(&mut self){
        unsafe{
            let layout = Layout::from_size_align(self.size, 8).unwrap();
            dealloc(self.vec, layout);
            self.vec = std::ptr::null_mut();
        }
    }

    /// # `get_vec`
    /// Returns a mutable pointer to the data block.
    /// 
    /// # Example
    /// ```rust
    /// use PTHome::main_code::data_structures::DataB;
    /// let db = DataB::new(1024).unwrap_or(assertion_failed!());
    /// let vec_ptr = db.get_vec();
    /// assert!(!vec_ptr.is_null());
    /// ```
    pub fn get_vec(&self)->*mut u8{
          self.vec  
    }

    /// # `set_vec_ptr`
    /// Sets the pointer to the data block, deallocating any existing memory.
    /// 
    /// # Arguments
    /// * `ptr` - A mutable pointer to the new data block.
    /// * `size` - Length of the new block in bytes.
    /// 
    /// # Example
    /// ```rust
    /// use PTHome::main_code::data_structures::DataB;
    /// let mut db = DataB::new(1024).unwrap_or(assertion_failed!());
    /// let new_ptr = unsafe { std::alloc::alloc(std::alloc::Layout::from_size_align(2048, 8).unwrap()) };
    /// db.set_vec_ptr(new_ptr);
    /// assert_eq!(db.get_vec(), new_ptr);
    /// asset_eq!(db.ptr(), 0); // reset the pointer
    /// ```
    pub fn set_vec_ptr(&mut self,ptr:*mut u8, size:usize){
          self.deallocate();
          self.vec = ptr;
          self.ptr = 0;
          self.size = size;
    }

    /// # `unsafe_set_vec_ptr`
    /// Sets the pointer to the data block without deallocating existing memory.
    /// 
    /// # Arguments
    /// * `ptr` - A mutable pointer to the new data block.
    /// * `size` - Length of the new block in bytes.
    /// 
    /// # Example
    /// ```rust
    /// use PTHome::main_code::data_structures::DataB;
    /// let mut db = DataB::new(1024).unwrap_or(assertion_failed!());
    /// let mut old = db.get_vec();
    /// let new_ptr = unsafe { std::alloc::alloc(std::alloc::Layout::from_size_align(2048, 8).unwrap()) };
    /// db.unsafe_set_vec_ptr(new_ptr);
    /// assert_eq!(db.get_vec(), new_ptr);
    /// assert_eq!(old, old); // old pointer is still valid
    /// asset_eq!(db.ptr(), 0); // reset the pointer
    /// ```
    pub fn unsafe_set_vec_ptr(&mut self, ptr:*mut u8, size:usize){
          self.vec = ptr;
          self.size = size;
          self.ptr = 0;
    }

    /// # `get_size`
    /// Returns the size of the data block in bytes.
    /// 
    /// # Example
    /// ```rust
    /// use PTHome::main_code::data_structures::DataB;
    /// let db = DataB::new(2048);
    /// assert_eq!(db.get_size(), 2048); 
    /// ```
    pub fn get_size(&self)->usize{
          self.size
    }
    
    /// # `ptr`
    /// Returns the current pointer value, which indicates the next bit free to use as a pointer of a stack.
    /// 
    /// # Example
    /// ```rust
    /// use PTHome::main_code::data_structures::DataB;
    /// let db = DataB::new(1024).unwrap_or(assertion_failed!());
    /// let chunk = db.ptr();
    /// assert_eq!(chunk, 0);
    /// ```
    pub fn ptr(&self)->usize{
      self.ptr
    }

    /// # `warm_up`
    /// This method is designed to "warm up" the memory by accessing each byte in the specified range to take it in the chache. This can help reduce latency for subsequent accesses to that memory.
    /// 
    /// # Arguments
    /// * `since_bit` - The starting bit index from which to begin warming up the memory.
    /// * `to_bit` - The ending bit index up to which to warm up the memory.
    /// 
    /// # Example
    /// ```rust
    /// use bitdata::bitdata::DataB;
    /// use std::time::Instant;
    /// let n = 1026; // Total bits in the block
    /// match DataB::new(n){
    /// Ok(db) => {
    /// let mut start = Instant::now();
    /// let mut duration;
    /// match db.warm_up(10, 1024) // Warm up the entire block (8 bits per byte)
    /// {
    /// Ok(()) => {duration = start.elapsed();
    /// println!("Time of warm up: {:?}", duration);},
    /// Err(e) => panic!("Warm up failed: {:?}", e),
    /// }
    /// start = Instant::now();
    /// unsafe{*db.get_vec().add(0) = 5;} 
    ///  duration = start.elapsed();
    ///  println!("Time without warm up: {:?}", duration);
    ///  
    /// start = Instant::now();
    /// unsafe{*db.get_vec().add(4) = 10;}
    /// duration = start.elapsed();
    /// println!("Time with warm up: {:?}", duration);},
    /// Err(e) => panic!("Failed to create DataB: {:?}", e),
    /// }
    /// ```
    /// 
    /// # Returns
    /// * `Ok(())` - if the warm-up process completes successfully.
    /// * `Err(AllocErr)` - if there is an error during the warm-up process, such as an unrecognized instruction or an out-of-bounds access.
    pub fn warm_up(&self, since_bit:usize, to_bit:usize)->Result<(),AllocErr>{
      let since = byte_from_bits(since_bit, false)?.0;
      let to = byte_from_bits(to_bit, false)?.0;
       if since > to {return Err(AllocErr::UnrecognizedInstruction);}
      let s = since as isize;
      for i in since..to{
        unsafe{
          let ptr = self.vec.offset(i as isize);
          *ptr = *ptr & 0xFF;
        }
      }
      Ok(())
    }

    /// # `push`
    /// Pushes a value into the data block at the specified bit index, with optional size and overflow management.
    /// 
    /// # Arguments
    /// * `bit_index` - The bit index at which to push the value.
    /// * `value` - The value to be pushed into the data block.
    /// * `size_in_bits` - An optional parameter specifying the size of the value in bits. If not provided, it defaults to the size of the value in bits.
    /// * `manage_overflow` - An optional parameter specifying how to manage overflow if the
    /// data block does not have enough space to accommodate the value.
    /// 
    /// # Example
    /// ```rust
    /// use bitdata::bitdata::DataB;
    /// let mut db = DataB::new(1024).unwrap_or(assertion_failed!());
    /// let value: u16 = 0xABCD;
    /// let size_in_bits = Some(12); // Only push the bigger 12 bits
    /// let (result, bit_size) = db.push(0, value, size_in_bits, None);
    /// match result {
    /// Ok(()) => println!("Value pushed successfully!"),
    /// Err(e) => println!("Failed to push value: {:?}", e),
    /// }
    /// ```
    /// 
    /// # Returns
    /// * A tuple containing:
    ///  * `Result<(), AllocErr>` - indicating whether the push operation was successful or if an error occurred.
    /// * `Option<usize>` - an optional value indicating the size in bits that was actually pushed, which will changed from the requested size if is greather 
    /// than the length of the value.
    /// 
    /// # IMPORTANT
    /// The `push` method insterts the datas in big-endian order, meaning that the most significant bits of the value will be pushed first, starting 
    /// from the specified bit index. If the `size_in_bits` parameter is provided and is less than the actual size of the value in bits, only the most 
    /// significant bits up to the specified size will be pushed into the data block.
  pub fn push<T:PushOptimized>(&mut self, bit_index:usize, value:T, size_in_bits:Option<usize>, manage_overflow:Option<OverFlow>)->(Result<(), AllocErr>, Option<usize>){
      let mut n = size_in_bits;
      let res = value.push_at_buffer(self, bit_index, &mut n, manage_overflow);
     if n!=size_in_bits{
       return (res, n);
     }
     (res, None)
  }

  /// # `get`
  /// Retrieves a value from the data block starting at the specified bit index and spanning the specified size in bits.
  /// 
  /// # Arguments
  /// * `bit_index_start` - The starting bit index from which to retrieve the value.
  /// * `len_in_bits` - The size of the value to be retrieved in bits.
  /// 
  /// # Example
  /// ```rust
  /// use bitdata::bitdata::DataB;
  /// let mut db = DataB::new(1024).unwrap_or(assertion_failed!());
   /* 
  pub fn get<T:ReadOptimized>(&self, bit_index_start:u64, len_in_bits:u64)->Result<[T], AllocErr>{
     ReadOptimized::read_from_buffer(&T, self, bit_index_start, len_in_bits)
  }*/
    /// # `move_ptr`
    /// Moves the internal pointer to the specified bit index, ensuring it does not exceed the total size of the data block in bits.
    /// 
    /// # Arguments
    /// * `bit` - The bit index to which the pointer should be moved.
    ///
    /// # Example
    /// ```rust
    /// use bitdata::bitdata::DataB;
    /// let mut db = DataB::new(1024).unwrap_or(assertion_failed!());
    /// db.move_ptr(512); // Move the pointer to the 512th bit
    /// assert_eq!(db.ptr(), 512);
    /// db.move_ptr(8192); // Attempt to move the pointer beyond the block size (1024 bytes = 8192 bits)
    /// assert_eq!(db.ptr(), 8192); // Pointer should be capped at the block size
    /// ```
   pub fn move_ptr(&mut self, bit: usize) {
    if bit > (self.size*8){self.ptr = (self.size*8);}
    else{self.ptr=bit;}
   }

}

  /// # `Drop` Implementation for DataB
  /// This implementation ensures that the allocated memory for the data block and auxiliary pointer block is properly deallocated when the DataB instance goes out of scope.
  /// 
  /// # Example
  /// ```rust
  /// use PTHome::main_code::data_structures::DataB;
  /// {
  /// let db = DataB::new(1024).unwrap_or(assertion_failed!());
  /// } // db is automatically dropped here, and memory is deallocated
  /// 
  /// ```
  impl Drop for DataB{
    fn drop(&mut self) {
        self.deallocate(); 
    }
  }
  use std::time::Instant;
use bitvec::vec::BitVec;
use std::hint::black_box;
/*
 */
#[cfg(test)]
#[test]
fn test_n() {
       let runs = 5usize;
    println!("=== Benchmark: DataB vs BitVec ===\n");

    // Test 1: 1-bit pushes (BitVec's strongest case)
    println!("TEST 1: Push 1-bit (BitVec's native case)");
    bench_1bit(1_000, runs);
    bench_1bit(10_000, runs);

    // Test 2: u8 pushes (8-bit typed)
    println!("\nTEST 2: Push u8 (8-bit native type)");
    bench_u8(1_000, runs);
    bench_u8(10_000, runs);

    // Test 3: u16 pushes (16-bit typed)
    println!("\nTEST 3: Push u16 (16-bit native type)");
    bench_u16(1_000, runs);
    bench_u16(10_000, runs);

    // Test 4: u32 pushes (32-bit typed)
    println!("\nTEST 4: Push u32 (32-bit native type)");
    bench_u32(1_000, runs);
    bench_u32(10_000, runs);

    // Test 5: u64 pushes (64-bit typed)
    println!("\nTEST 5: Push u64 (64-bit native type)");
    bench_u64(1_000, runs);
    bench_u64(10_000, runs);

    // Test 6: u128 pushes (128-bit typed)
    println!("\nTEST 6: Push u128 (128-bit native type)");
    bench_u128(1_000, runs);
    bench_u128(10_000, runs);

    // Test 7: 24-bit custom size
    println!("\nTEST 7: Push 24-bit custom size");
    bench_24bit(1_000, runs);
    bench_24bit(10_000, runs);

    // Test 8: Mixed sizes (simulating real-world usage)
    println!("\nTEST 8: Push mixed sizes (12, 16, 24, 32 bits)");
    bench_mixed(1_000, runs);
    bench_mixed(10_000, runs);
}

fn bench_1bit(n: usize, runs: usize) {
    let mut times_db = Vec::new();
    let mut times_bv = Vec::new();

    for _ in 0..runs {
        let start = Instant::now();
        let mut db = DataB::new((n / 8) + 8).expect("alloc");
        for i in 0..n {
            let _ = db.push(i, black_box(1u8), Some(1), None);
        }
        black_box(db.ptr());
        db.deallocate();
        times_db.push(start.elapsed().as_nanos());

        let start = Instant::now();
        let mut bv: BitVec = BitVec::with_capacity(n);
        for _ in 0..n {
            bv.push(black_box(true));
        }
        black_box(&bv);
        times_bv.push(start.elapsed().as_nanos());
    }

    let avg_db = times_db.iter().sum::<u128>() / times_db.len() as u128;
    let avg_bv = times_bv.iter().sum::<u128>() / times_bv.len() as u128;
    let ratio = avg_bv as f64 / avg_db as f64;
    println!("  n={}: DataB={} ns, BitVec={} ns | BitVec {:.2}x", n, avg_db, avg_bv, ratio);
}

fn bench_u8(n: usize, runs: usize) {
    let mut times_db = Vec::new();
    let mut times_bv = Vec::new();

    for _ in 0..runs {
        let start = Instant::now();
        let mut db = DataB::new(n + 8).expect("alloc");
        for i in 0..n {
            let _ = db.push(i, black_box((i & 0xFF) as u8), None, None);
        }
        black_box(db.ptr());
        db.deallocate();
        times_db.push(start.elapsed().as_nanos());
        //println!("data:{:?}", db.get(0, 8).unwrap());
        let start = Instant::now();
        let mut bv: BitVec = BitVec::with_capacity(n * 8);
        for i in 0..n {
            let byte = (i & 0xFF) as u8;
            // Push 8 bits in sequence
            bv.push((byte & 0x01) != 0);
            bv.push((byte & 0x02) != 0);
            bv.push((byte & 0x04) != 0);
            bv.push((byte & 0x08) != 0);
            bv.push((byte & 0x10) != 0);
            bv.push((byte & 0x20) != 0);
            bv.push((byte & 0x40) != 0);
            bv.push((byte & 0x80) != 0);
        }
        black_box(&bv);
        times_bv.push(start.elapsed().as_nanos());
    }

    let avg_db = times_db.iter().sum::<u128>() / times_db.len() as u128;
    let avg_bv = times_bv.iter().sum::<u128>() / times_bv.len() as u128;
    let ratio = avg_bv as f64 / avg_db as f64;
    println!("  n={}: DataB={} ns, BitVec={} ns | BitVec {:.2}x", n, avg_db, avg_bv, ratio);
}

fn bench_u16(n: usize, runs: usize) {
    let mut times_db = Vec::new();
    let mut times_bv = Vec::new();

    for _ in 0..runs {
        let start = Instant::now();
        let mut db = DataB::new((n * 2)+ 8).expect("alloc");
        for i in 0..n {
            let _ = db.push(i, black_box((i & 0xFFFF) as u16), Some(16), None);
        }
        black_box(db.ptr());
        db.deallocate();
        times_db.push(start.elapsed().as_nanos());

        let start = Instant::now();
        let mut bv: BitVec = BitVec::with_capacity(n * 16);
        for i in 0..n {
            let val = (i & 0xFFFF) as u16;
            for b in 0u32..16 {
                bv.push(((val >> b) & 1) == 1);
            }
        }
        black_box(&bv);
        times_bv.push(start.elapsed().as_nanos());
    }

    let avg_db = times_db.iter().sum::<u128>() / times_db.len() as u128;
    let avg_bv = times_bv.iter().sum::<u128>() / times_bv.len() as u128;
    let ratio = avg_bv as f64 / avg_db as f64;
    println!("  n={}: DataB={} ns, BitVec={} ns | BitVec {:.2}x", n, avg_db, avg_bv, ratio);
}

fn bench_u32(n: usize, runs: usize) {
    let mut times_db = Vec::new();
    let mut times_bv = Vec::new();

    for _ in 0..runs {
        let start = Instant::now();
        let mut db = DataB::new((n * 4) + 8).expect("alloc");
        for i in 0..n {
            let _ = db.push(i, black_box(i as u32), Some(32), None);
        }
        black_box(db.ptr());
        db.deallocate();
        times_db.push(start.elapsed().as_nanos());

        let start = Instant::now();
        let mut bv: BitVec = BitVec::with_capacity(n * 32);
        for i in 0..n {
            let val = i as u32;
            for b in 0u32..32 {
                bv.push(((val >> b) & 1) == 1);
            }
        }
        black_box(&bv);
        times_bv.push(start.elapsed().as_nanos());
    }

    let avg_db = times_db.iter().sum::<u128>() / times_db.len() as u128;
    let avg_bv = times_bv.iter().sum::<u128>() / times_bv.len() as u128;
    let ratio = avg_bv as f64 / avg_db as f64;
    println!("  n={}: DataB={} ns, BitVec={} ns | BitVec {:.2}x", n, avg_db, avg_bv, ratio);
}

fn bench_u64(n: usize, runs: usize) {
    let mut times_db = Vec::new();
    let mut times_bv = Vec::new();

    for _ in 0..runs {
        let start = Instant::now();
        let mut db = DataB::new((n * 8) + 8).expect("alloc");
        for i in 0..n {
            let _ = db.push(i, black_box(i as u64), Some(64), None);
        }
        black_box(db.ptr());
        db.deallocate();
        times_db.push(start.elapsed().as_nanos());

        let start = Instant::now();
        let mut bv: BitVec = BitVec::with_capacity(n * 64);
        for i in 0..n {
            let val = i;
            for b in 0u32..64 {
                bv.push(((val >> b) & 1) == 1);
            }
        }
        black_box(&bv);
        times_bv.push(start.elapsed().as_nanos());
    }

    let avg_db = times_db.iter().sum::<u128>() / times_db.len() as u128;
    let avg_bv = times_bv.iter().sum::<u128>() / times_bv.len() as u128;
    let ratio = avg_bv as f64 / avg_db as f64;
    println!("  n={}: DataB={} ns, BitVec={} ns | BitVec {:.2}x", n, avg_db, avg_bv, ratio);
}

fn bench_24bit(n: usize, runs: usize) {
    let mut times_db = Vec::new();
    let mut times_bv = Vec::new();

    for _ in 0..runs {
        let start = Instant::now();
        let mut db = DataB::new((n * 3) + 8).expect("alloc");
        for i in 0..n {
            let val = (i & 0xFFFFFF) as u32;
            let _ = db.push(i, black_box(val), Some(24), None);
        }
        black_box(db.ptr());
        db.deallocate();
        times_db.push(start.elapsed().as_nanos());

        let start = Instant::now();
        let mut bv: BitVec = BitVec::with_capacity(n * 24);
        for i in 0..n {
            let val = (i & 0xFFFFFF) as u32;
            for b in 0u32..24 {
                bv.push(((val >> b) & 1) == 1);
            }
        }
        black_box(&bv);
        times_bv.push(start.elapsed().as_nanos());
    }

    let avg_db = times_db.iter().sum::<u128>() / times_db.len() as u128;
    let avg_bv = times_bv.iter().sum::<u128>() / times_bv.len() as u128;
    let ratio = avg_bv as f64 / avg_db as f64;
    println!("  n={}: DataB={} ns, BitVec={} ns | BitVec {:.2}x", n, avg_db, avg_bv, ratio);
}

fn bench_mixed(n: usize, runs: usize) {
    let mut times_db = Vec::new();
    let mut times_bv = Vec::new();

    for _ in 0..runs {
        let start = Instant::now();
        let mut db = DataB::new(n * 2 + 8).expect("alloc");
        for i in 0..n {
            match i % 4 {
                0 => { let _ = db.push(i, black_box((i & 0xFFF) as u16), Some(12), None); },
                1 => { let _ = db.push(i, black_box((i & 0xFFFF) as u16), Some(16), None); },
                2 => { let _ = db.push(i, black_box((i & 0xFFFFFF) as u32), Some(24), None); },
                _ => { let _ = db.push(i , black_box(i as u32), Some(32), None); },
            }
        }
        black_box(db.ptr());
        db.deallocate();
        times_db.push(start.elapsed().as_nanos());

        let start = Instant::now();
        let mut bv: BitVec = BitVec::with_capacity(n * 21); // avg of 12,16,24,32
        for i in 0..n {
            let (val, bits) = match i % 4 {
                0 => ((i & 0xFFF), 12u32),
                1 => ((i & 0xFFFF), 16u32),
                2 => ((i & 0xFFFFFF), 24u32),
                _ => (i, 32u32),
            };
            for b in 0..bits {
                bv.push(((val >> b) & 1) == 1);
            }
        }
        black_box(&bv);
        times_bv.push(start.elapsed().as_nanos());
    }

    let avg_db = times_db.iter().sum::<u128>() / times_db.len() as u128;
    let avg_bv = times_bv.iter().sum::<u128>() / times_bv.len() as u128;
    let ratio = avg_bv as f64 / avg_db as f64;
    println!("  n={}: DataB={} ns, BitVec={} ns | BitVec {:.2}x", n, avg_db, avg_bv, ratio);
  }

fn bench_u128(n:usize, runs:usize){
  let mut times_db = Vec::new();
    let mut times_bv = Vec::new();

    for _ in 0..runs {
        let start = Instant::now();
        let mut db = DataB::new((n * 16) + 8).expect("alloc");
        for i in 0..n {
            let _ = db.push(i, black_box(i as u128), Some(128), None);
        }
        black_box(db.ptr());
        db.deallocate();
        times_db.push(start.elapsed().as_nanos());

        let start = Instant::now();
        let mut bv: BitVec = BitVec::with_capacity(n * 128);
        for i in 0..n {
            let val = i as u128;
            for b in 0u32..128 {
                bv.push(((val >> b) & 1) == 1);
            }
        }
        black_box(&bv);
        times_bv.push(start.elapsed().as_nanos());
    }

    let avg_db = times_db.iter().sum::<u128>() / times_db.len() as u128;
    let avg_bv = times_bv.iter().sum::<u128>() / times_bv.len() as u128;
    let ratio = avg_bv as f64 / avg_db as f64;
    println!("  n={}: DataB={} ns, BitVec={} ns | BitVec {:.2}x", n, avg_db, avg_bv, ratio);
}
//------------------------------------------------------------------
#[repr(C)]

/// # `B15`
/// B15 is a data structure with separate data, addresses, and mapping blocks.
/// 
/// # Fields
/// * `data_block:DataB` - This block has to store the datas.
/// * `addr_block:DataB` - This block has to store the addresses, that are pointers to others data blocks, and mapper blocks.
/// * `mapper_block:DataB` - This block has to store the mapping information for data organization and retrieval.
pub struct B15{
      addr_block:(*mut u8, DataB),
}
impl B15{
    /// Creates a new B15 instance.
    /// 
    /// # Arguments
    /// * `data_block_size` - The size of the block in bytes. If 0, defaults to 15360 (15kB).
    /// * `addresses_block_size` - The size of the addresser block in bytes. If 0, defaults to 4096 (4kB).
    /// * `mapper_block_size` - The size of the mapper block in bytes. If 0, defaults to 10240 (10kB).
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use PTHome::main_code::data_structures::B15;
    /// let b = B15::new(0);
    /// assert_eq!(b.b_size(),15360);
    /// ```
    /// 
    /// # Returns
    /// * A new instance of b15 with the specified block size.
    /*
    pub fn new(data_block_size:usize, addresser_block_size:usize, mapper_block_size:usize)->Self{
      if data_block_size == 0{
        if addresser_block_size ==0{
          if mapper_block_size ==0{
            
            let mut next;
            let mut layout;
            // Reserve memory for another addr_block
            unsafe {
             layout = std::alloc::Layout::from_size_align(4096, 8).unwrap();
             next = std::alloc::alloc(layout);
            }
            // Create the B15 instance with the new addr_block
            let mut r = B15{
              addr_block:(next, DataB::new(4096, false)),
            };

            let mapper_vec = DataB::new(10240, false); // Mapper block
            let data_vec = DataB::new(15360, false); // Data block

            let mut ptr = r.addr_block.1.get_vec();
            let offset = r.add_block_1.ptr();
            *ptr.add() = 
            
          }else{
            B15{
              data_block:DataB::new(15360),
              addr_block:DataB::new(4096),
              mapper_block:DataB::new(mapper_block_size),
            }
          }
        }else{
          if mapper_block_size ==0{
            B15{
              data_block:DataB::new(15360),
              addr_block:DataB::new(addresser_block_size),
              mapper_block:DataB::new(10240),
            }
          }else{
            B15{
              data_block:DataB::new(15360),
              addr_block:DataB::new(addresser_block_size),
              mapper_block:DataB::new(mapper_block_size),
            }
          }
        }
      }else{
        if addresser_block_size ==0{
          if mapper_block_size ==0{
            B15{
              data_block:DataB::new(data_block_size),
              addr_block:DataB::new(4096),
              mapper_block:DataB::new(10240),
            }
          }else{
            B15{
              data_block:DataB::new(data_block_size),
              addr_block:DataB::new(4096),
              mapper_block:DataB::new(mapper_block_size),
            }
          }
        }else{
          if mapper_block_size ==0{
            B15{
              data_block:DataB::new(data_block_size),
              addr_block:DataB::new(addresser_block_size),
              mapper_block:DataB::new(10240),
            }
          }else{
            B15{
              data_block:DataB::new(data_block_size),
              addr_block:DataB::new(addresser_block_size),
              mapper_block:DataB::new(mapper_block_size),
            }
          }
        }
      }
    }
*/
  pub fn push<T>(&mut self, data:&T){
    let mut padding = true;
    for byte in Self::byte(data).iter(){
      for j in 0..8{
        if padding{
          let bit = (byte<<j) & 0b10000000;
          if bit != 0{
            padding =false;
          }
        }else{

        }
      }
    }
  }

    pub fn byte<T>(data:&T)->&[u8]{
       unsafe {
        std::slice::from_raw_parts(
            data as *const T as *const u8,
            std::mem::size_of::<T>()
        )
     }
    }

     

}