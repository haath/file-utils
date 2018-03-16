
use std::io;
use std::fs::File;

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

	fn write_isize(&mut self, val: isize)-> io::Result<()>
	{
		self.write_usize(val as usize)
	}

	/*
		8-bit
	*/
	fn write_u8(&mut self, val: u8)-> io::Result<()>
	{
		use std::io::Write;
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
		use std::io::Write;
		self.write_all(&u16_to_bytes(val))
	}

	fn write_i16(&mut self, val: i16)-> io::Result<()>
	{
		self.write_u16(val as u16)
	}

	/*
		32-bit
	*/
	fn write_u32(&mut self, val: u32)-> io::Result<()>
	{
		use std::io::Write;
		self.write_all(&u32_to_bytes(val))
	}

	fn write_i32(&mut self, val: i32)-> io::Result<()>
	{
		self.write_u32(val as u32)
	}
	
	fn write_f32(&mut self, val: f32)-> io::Result<()>
	{
		self.write_u32(val as u32)
	}

	/*
		64-bit
	*/
	fn write_u64(&mut self, val: u64)-> io::Result<()>
	{
		use std::io::Write;
		self.write_all(&u64_to_bytes(val))
	}

	fn write_i64(&mut self, val: i64)-> io::Result<()>
	{
		self.write_u64(val as u64)
	}
	
	fn write_f64(&mut self, val: f64)-> io::Result<()>
	{
		self.write_u64(val as u64)
	}
}

#[cfg(target_endian="big")]
fn u16_to_bytes(x: u16) -> [u8; 2]
{
    [
    	((x >>  8) & 0xff) as u8,
    	((x >>  0) & 0xff) as u8
	]
}

#[cfg(target_endian="little")]
fn u16_to_bytes(x: u16) -> [u8; 2]
{
    [
		((x >>  0) & 0xff) as u8,
    	((x >>  8) & 0xff) as u8
	]
}

#[cfg(target_endian="big")]
fn u32_to_bytes(x: u32) -> [u8; 4]
{
    [
		((x >> 24) & 0xff) as u8,
    	((x >> 16) & 0xff) as u8,
    	((x >>  8) & 0xff) as u8,
    	((x >>  0) & 0xff) as u8
	]
}

#[cfg(target_endian="little")]
fn u32_to_bytes(x: u32) -> [u8; 4]
{
    [
		((x >>  0) & 0xff) as u8,
    	((x >>  8) & 0xff) as u8,
    	((x >> 16) & 0xff) as u8,
    	((x >> 24) & 0xff) as u8
	]
}

#[cfg(target_endian="big")]
fn u64_to_bytes(x: u64) -> [u8; 8]
{
    [
		((x >> 56) & 0xff) as u8,
    	((x >> 48) & 0xff) as u8,
    	((x >> 40) & 0xff) as u8,
    	((x >> 32) & 0xff) as u8,
		((x >> 24) & 0xff) as u8,
    	((x >> 16) & 0xff) as u8,
    	((x >>  8) & 0xff) as u8,
    	((x >>  0) & 0xff) as u8
	]
}

#[cfg(target_endian="little")]
fn u64_to_bytes(x: u64) -> [u8; 8]
{
    [
		((x >>  0) & 0xff) as u8,
    	((x >>  8) & 0xff) as u8,
    	((x >> 16) & 0xff) as u8,
    	((x >> 24) & 0xff) as u8,
		((x >> 32) & 0xff) as u8,
    	((x >> 40) & 0xff) as u8,
    	((x >> 48) & 0xff) as u8,
    	((x >> 56) & 0xff) as u8
	]
}