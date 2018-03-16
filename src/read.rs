
use std::io;
use std::fs::File;


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
	fn read_f32(&mut self)-> io::Result<f32>;
	fn read_i32(&mut self)-> io::Result<i32>;

	// 64-bit
	fn read_u64(&mut self)-> io::Result<u64>;
	fn read_f64(&mut self)-> io::Result<f64>;
	fn read_i64(&mut self)-> io::Result<i64>;
}

impl Read for File
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

	fn read_isize(&mut self) -> io::Result<isize>
	{
		let num = self.read_usize()?;
		Ok(num as isize)
	}

	/*
		8-bit
	*/
	fn read_u8(&mut self)-> io::Result<u8>
	{
		use std::io::Read;
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
		use std::io::Read;
		let mut buf: [u8; 2] = [0; 2];
		self.read_exact(&mut buf)?;
		Ok(bytes_to_u16(buf))
	}

	fn read_i16(&mut self)-> io::Result<i16>
	{
		let num = self.read_u16()?;
		Ok(num as i16)
	}

	/*
		32-bit
	*/
	fn read_u32(&mut self)-> io::Result<u32>
	{
		use std::io::Read;
		let mut buf: [u8; 4] = [0; 4];
		self.read_exact(&mut buf)?;
		Ok(bytes_to_u32(buf))
	}

	fn read_f32(&mut self)-> io::Result<f32>
	{
		let num = self.read_u32()?;
		Ok(num as f32)
	}

	fn read_i32(&mut self)-> io::Result<i32>
	{
		let num = self.read_u32()?;
		Ok(num as i32)
	}

	/*
		64-bit
	*/
	fn read_u64(&mut self)-> io::Result<u64>
	{
		use std::io::Read;
		let mut buf: [u8; 8] = [0; 8];
		self.read_exact(&mut buf)?;
		Ok(bytes_to_u64(buf))
	}

	fn read_f64(&mut self)-> io::Result<f64>
	{
		let num = self.read_u64()?;
		Ok(num as f64)
	}

	fn read_i64(&mut self)-> io::Result<i64>
	{
		let num = self.read_u64()?;
		Ok(num as i64)
	}
}



#[cfg(target_endian="big")]
pub fn bytes_to_u16(x: [u8; 2]) -> u16
{
    ((x[0] as u16) <<  8) +
    ((x[1] as u16) <<  0)
}

#[cfg(target_endian="little")]
pub fn bytes_to_u16(x: [u8; 2]) -> u16
{
    ((x[1] as u16) <<  8) +
    ((x[0] as u16) <<  0)
}

#[cfg(target_endian="big")]
pub fn bytes_to_u32(x: [u8; 4]) -> u32
{
    ((x[0] as u32) << 24) +
    ((x[1] as u32) << 16) +
    ((x[2] as u32) <<  8) +
    ((x[3] as u32) <<  0)
}

#[cfg(target_endian="little")]
pub fn bytes_to_u32(x: [u8; 4]) -> u32
{
    ((x[3] as u32) << 24) +
    ((x[2] as u32) << 16) +
    ((x[1] as u32) <<  8) +
    ((x[0] as u32) <<  0)
}

#[cfg(target_endian="big")]
pub fn bytes_to_u64(x: [u8; 8]) -> u64
{
    ((x[0] as u64) << 56) +
    ((x[1] as u64) << 48) +
    ((x[2] as u64) << 40) +
    ((x[3] as u64) << 32) +
    ((x[4] as u64) << 24) +
    ((x[5] as u64) << 16) +
    ((x[6] as u64) <<  8) +
    ((x[7] as u64) <<  0)
}

#[cfg(target_endian="little")]
pub fn bytes_to_u64(x: [u8; 8]) -> u64
{
    ((x[7] as u64) << 56) +
    ((x[6] as u64) << 48) +
    ((x[5] as u64) << 40) +
    ((x[4] as u64) << 32) +
    ((x[3] as u64) << 24) +
    ((x[2] as u64) << 16) +
    ((x[1] as u64) <<  8) +
    ((x[0] as u64) <<  0)
}