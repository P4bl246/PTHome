#![allow(dead_code)]
#![allow(unused_imports)]
pub struct B15{
      block_sz:usize,
}
impl B15{
    /// Creates a new b15 instance.
    /// 
    /// # Arguments
    /// * `block_size` - The size of the block in bytes. If 0, defaults to 15360 (15kB).
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
    pub fn new(block_size:usize)->Self{
      if block_size == 0{
        B15{
           
          block_sz:15360,
        }
      }else{
        B15{
           
          block_sz:block_size,
        }
      }
    }

    /// Returns the block size in bytes.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use PTHome::main_code::data_structures::B15;
    /// let b = B15::new(20);
    /// assert_eq!(b.b_size(),20);
    /// ```
    pub fn b_size(&self)->usize{
      self.block_sz
    }

    


}