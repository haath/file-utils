
use std::io;

use conversions::u::*;
use conversions::i::*;

pub trait Read
{
	// Conditional
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
}

impl<T> Read for T 
	where T: io::Read
{
	/*
		Based on architecture
	*/
	#[cfg(target_pointer_width = "64")]
	fn read_usize(&mut self)-> io::Result<usize>
	{
		let num = self.read_u64()?;
		Ok(num as usize)
	}

	#[cfg(target_pointer_width = "32")]
	fn read_usize(&mut self)-> io::Result<usize>
	{
		let num = self.read_u32()?;
		Ok(num as usize)
	}

	#[cfg(target_pointer_width = "64")]
	fn read_isize(&mut self) -> io::Result<isize>
	{
		let num = self.read_i64()?;
		Ok(num as isize)
	}

	#[cfg(target_pointer_width = "32")]
	fn read_isize(&mut self) -> io::Result<isize>
	{
		let num = self.read_i32()?;
		Ok(num as isize)
	}

	/*
		8-bit
	*/
	fn read_u8(&mut self)-> io::Result<u8>
	{
		let mut buf: [u8; 1] = [0; 1];
		self.read_exact(&mut buf)?;
		Ok(buf[0])
	}

	fn read_i8(&mut self)-> io::Result<i8>
	{
		let num = self.read_u8()?;
		Ok(num as i8)
	}

	/*
		16-bit
	*/
	fn read_u16(&mut self)-> io::Result<u16>
	{
		let mut buf: [u8; 2] = [0; 2];
		self.read_exact(&mut buf)?;
		Ok(bytes_to_u16(&buf))
	}

	fn read_i16(&mut self)-> io::Result<i16>
	{
		let mut buf: [u8; 2] = [0; 2];
		self.read_exact(&mut buf)?;
		Ok(bytes_to_i16(&buf))
	}

	/*
		32-bit
	*/
	fn read_u32(&mut self)-> io::Result<u32>
	{
		let mut buf: [u8; 4] = [0; 4];
		self.read_exact(&mut buf)?;
		Ok(bytes_to_u32(&buf))
	}

	fn read_i32(&mut self)-> io::Result<i32>
	{
		let mut buf: [u8; 4] = [0; 4];
		self.read_exact(&mut buf)?;
		Ok(bytes_to_i32(&buf))
	}

	fn read_f32(&mut self)-> io::Result<f32>
	{
		let num = self.read_u32()?;
		Ok(num as f32)
	}

	/*
		64-bit
	*/
	fn read_u64(&mut self)-> io::Result<u64>
	{
		let mut buf: [u8; 8] = [0; 8];
		self.read_exact(&mut buf)?;
		Ok(bytes_to_u64(&buf))
	}

	fn read_i64(&mut self)-> io::Result<i64>
	{
		let mut buf: [u8; 8] = [0; 8];
		self.read_exact(&mut buf)?;
		Ok(bytes_to_i64(&buf))
	}

	fn read_f64(&mut self)-> io::Result<f64>
	{
		let num = self.read_u64()?;
		Ok(num as f64)
	}
}


