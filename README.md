# HELLO AGAIN!
Now we'll talk about where you are now in PTHome and a little bit of the next steps to help you to remember it.

## Right now:
1. You are making **DataB** in the **bitdata** crate that would be a tool to store data at bit-level:
   - You are develop the read function that will help you to check if the push function works well and add other to read the datas in a raw way without processing the datas to get.
   - You should to add a slice push function too, and add optimizations with SIMD in all the push functions and read functions too.
   - You must to add documentation code and tests.
     
2. Then you must to do b15 with this DataB struct that give you all the foundations to make it and package data at bit-level and deduplication optimiztions and others optimizations, in shorts arrays that must to fits in the cache, and has 3 blocks design:
   - Th 1st is the data table or data-block in where you push the real data to be stored in b15, this must to be a DataB data type.
   - The 2nd table is the mapper-block that will store the metadatas that you need to get a value or the data stored in that data table (might for each data-table or might optimize it sharing mapper-blocks for data-tables if it works).
   - The 3rd table is the addres-block that will store at the beginning the start of the next addres-block when the actual is full and then the addresses to the data-blocks and the linked mapper-block to it data-block.
  
3. Then You'll return to make PTHome when you do it b15 and use it as the principal data structure, where you are now creating the formats first to create then the multiplexor system, and you must to add documentation,
   tests, optimizations, and the other features of PTHome.

# IMPORTANT:
-Remember that PTHome has a phase of compile the input grammar, to analize the inputs, so you need to build, the runtime engine, and the compile engine.

# ISSUES OR INCOMPLETE
- Defined all the bytecode that the grammar will be compiled.
- Documentation and tests.
