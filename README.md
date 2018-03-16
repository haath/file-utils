# file-utils

This crate aims to provide convenient one-liners for file I/O operations that carry no dependencies and don't require unsafe code.

Furthermore, to ensure that multi-byte primitive types and pointers like `usize` are encoded correctly, compilation of the crate will take into account:

- Architecture (32-bit vs 64-bit)
- Endianness


## Usage

All the methods are implemented directly for the `File` type, so all you need to do is bring the traits into scope.

```rust
extern crate file-utils;

use file-utils::read::Read;
use file-utils::write::Write;
```

### Reading binary

The following implementations have been made on the `File` type:

```rust
fn read_usize(&mut self)-> io::Result<usize>;
fn read_isize(&mut self)-> io::Result<isize>;

// 8-bit
fn read_u8(&mut self)-> io::Result<u8>;
fn read_i8(&mut self)-> io::Result<i8>;

// 16-bit
fn read_u16(&mut self)-> io::Result<u16>;
fn read_i16(&mut self)-> io::Result<i16>;

// 32-bit
fn read_u32(&mut self)-> io::Result<u32>;
fn read_f32(&mut self)-> io::Result<f32>;
fn read_i32(&mut self)-> io::Result<i32>;

// 64-bit
fn read_u64(&mut self)-> io::Result<u64>;
fn read_f64(&mut self)-> io::Result<f64>;
fn read_i64(&mut self)-> io::Result<i64>;
```



