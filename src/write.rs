
use std::io;

use conversions::u::*;
use conversions::i::*;
use conversions::f::*;

pub trait Write
{
	// Conditional
	fn write_usize(&mut self, val: usize)-> io::Result<()>;
	fn write_isize(&mut self, val: isize)-> io::Result<()>;

	// 8-bit
	fn write_u8(&mut self, val: u8)-> io::Result<()>;
	fn write_i8(&mut self, val: i8)-> io::Result<()>;

	// 16-bit
	fn write_u16(&mut self, val: u16)-> io::Result<()>;
	fn write_i16(&mut self, val: i16)-> io::Result<()>;

	// 32-bit
	fn write_u32(&mut self, val: u32)-> io::Result<()>;
	fn write_i32(&mut self, val: i32)-> io::Result<()>;
	fn write_f32(&mut self, val: f32)-> io::Result<()>;

	// 64-bit
	fn write_u64(&mut self, val: u64)-> io::Result<()>;
	fn write_i64(&mut self, val: i64)-> io::Result<()>;
	fn write_f64(&mut self, val: f64)-> io::Result<()>;
}

impl<T> Write for T 
	where T: io::Write
{
	/*
		Based on architecture
	*/
	#[cfg(target_pointer_width = "64")]
	fn write_usize(&mut self, val: usize)-> io::Result<()>
	{
		self.write_u64(val as u64)
	}

	#[cfg(target_pointer_width = "32")]
	fn write_usize(&mut self, val: usize)-> io::Result<()>
	{
		self.write_u32(val as u32)
	}

	#[cfg(target_pointer_width = "64")]
	fn write_isize(&mut self, val: isize)-> io::Result<()>
	{
		self.write_i64(val as i64)
	}

	#[cfg(target_pointer_width = "32")]
	fn write_isize(&mut self, val: isize)-> io::Result<()>
	{
		self.write_i32(val as i32)
	}

	/*
		8-bit
	*/
	fn write_u8(&mut self, val: u8)-> io::Result<()>
	{
		self.write_all(&[val])
	}

	fn write_i8(&mut self, val: i8)-> io::Result<()>
	{
		self.write_all(&[val as u8])
	}

	/*
		16-bit
	*/
	fn write_u16(&mut self, val: u16)-> io::Result<()>
	{
		self.write_all(&u16_to_bytes(val))
	}

	fn write_i16(&mut self, val: i16)-> io::Result<()>
	{
		self.write_all(&i16_to_bytes(val))
	}

	/*
		32-bit
	*/
	fn write_u32(&mut self, val: u32)-> io::Result<()>
	{
		self.write_all(&u32_to_bytes(val))
	}

	fn write_i32(&mut self, val: i32)-> io::Result<()>
	{
		self.write_all(&i32_to_bytes(val))
	}
	
	fn write_f32(&mut self, val: f32)-> io::Result<()>
	{
		self.write_all(&f32_to_bytes(val))
	}

	/*
		64-bit
	*/
	fn write_u64(&mut self, val: u64)-> io::Result<()>
	{
		self.write_all(&u64_to_bytes(val))
	}

	fn write_i64(&mut self, val: i64)-> io::Result<()>
	{
		self.write_all(&i64_to_bytes(val))
	}
	
	fn write_f64(&mut self, val: f64)-> io::Result<()>
	{
		self.write_all(&f64_to_bytes(val))
	}
}