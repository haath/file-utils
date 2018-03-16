# file-utils

[![crates.io](https://img.shields.io/crates/v/file-utils.svg)](https://crates.io/crates/file-utils)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
![Build Status](https://devops.gmantaos.com/buildStatus/icon?job=file-utils)

This crate aims to provide convenient one-liners for file I/O operations that carry no dependencies and don't require unsafe code.

Furthermore, to ensure that multi-byte primitive types and pointers like `usize` are encoded correctly, compilation of the crate will take into account:

- Architecture (32-bit vs 64-bit)
- Endianness


## Usage


### Reading binary

All the methods are implemented directly for any type that implements the `Read` trait, so all you need to do is bring the traits into scope.

```rust
extern crate file_utils;

use std::io;
use std::fs::File;

use file_utils::read::Read;		// <--- bring the Read trait into scope

fn foo() -> io::Result<()>
{
	let file = File::open("binary-file")?;

	// Read the first 8 bytes of the file into a u64
	let uns: u64 = file.read_u64()?;

	// And the next 4 bytes into an f32
	let flt: f32 = file.read_f32()?;

	Ok(())
}
```

All primitive number types can be read this way

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
fn read_i32(&mut self)-> io::Result<i32>;
fn read_f32(&mut self)-> io::Result<f32>;

// 64-bit
fn read_u64(&mut self)-> io::Result<u64>;
fn read_i64(&mut self)-> io::Result<i64>;
fn read_f64(&mut self)-> io::Result<f64>;
```

### Writing binary

Similarly to `Read`, this crate provides its writing methods as implementations to any type that implements `Write`.

```rust
extern crate file_utils;

use std::io;
use std::fs::File;

use file_utils::write::Write;		// <--- bring the Write trait into scope

fn foo() -> io::Result<()>
{
	let file = File::create("binary-file")?;

	// Write a usize to the file, which will either be 4 or 8 bytes depending on architecture
	file.write_usize(12)?;

	// Write a 4-byte floating point number
	file.write_f32(1.42)?;

	Ok(())
}
```

