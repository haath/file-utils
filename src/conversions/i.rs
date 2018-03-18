#[cfg(target_endian="big")]
pub fn bytes_to_i16(x: &[u8; 2]) -> i16
{
    ((x[0] as i16) <<  8) +
    ((x[1] as i16) <<  0)
}

#[cfg(target_endian="little")]
pub fn bytes_to_i16(x: &[u8; 2]) -> i16
{
    ((x[1] as i16) <<  8) +
    ((x[0] as i16) <<  0)
}

#[cfg(target_endian="big")]
pub fn bytes_to_i32(x: &[u8; 4]) -> i32
{
    ((x[0] as i32) << 24) +
    ((x[1] as i32) << 16) +
    ((x[2] as i32) <<  8) +
    ((x[3] as i32) <<  0)
}

#[cfg(target_endian="little")]
pub fn bytes_to_i32(x: &[u8; 4]) -> i32
{
    ((x[3] as i32) << 24) +
    ((x[2] as i32) << 16) +
    ((x[1] as i32) <<  8) +
    ((x[0] as i32) <<  0)
}

#[cfg(target_endian="big")]
pub fn bytes_to_i64(x: &[u8; 8]) -> i64
{
    ((x[0] as i64) << 56) +
    ((x[1] as i64) << 48) +
    ((x[2] as i64) << 40) +
    ((x[3] as i64) << 32) +
    ((x[4] as i64) << 24) +
    ((x[5] as i64) << 16) +
    ((x[6] as i64) <<  8) +
    ((x[7] as i64) <<  0)
}

#[cfg(target_endian="little")]
pub fn bytes_to_i64(x: &[u8; 8]) -> i64
{
    ((x[7] as i64) << 56) +
    ((x[6] as i64) << 48) +
    ((x[5] as i64) << 40) +
    ((x[4] as i64) << 32) +
    ((x[3] as i64) << 24) +
    ((x[2] as i64) << 16) +
    ((x[1] as i64) <<  8) +
    ((x[0] as i64) <<  0)
}


#[cfg(target_endian="big")]
pub fn i16_to_bytes(x: i16) -> [u8; 2]
{
    [
    	((x >>  8) & 0xff) as u8,
    	((x >>  0) & 0xff) as u8
	]
}

#[cfg(target_endian="little")]
pub fn i16_to_bytes(x: i16) -> [u8; 2]
{
    [
		((x >>  0) & 0xff) as u8,
    	((x >>  8) & 0xff) as u8
	]
}

#[cfg(target_endian="big")]
pub fn i32_to_bytes(x: i32) -> [u8; 4]
{
    [
		((x >> 24) & 0xff) as u8,
    	((x >> 16) & 0xff) as u8,
    	((x >>  8) & 0xff) as u8,
    	((x >>  0) & 0xff) as u8
	]
}

#[cfg(target_endian="little")]
pub fn i32_to_bytes(x: i32) -> [u8; 4]
{
    [
		((x >>  0) & 0xff) as u8,
    	((x >>  8) & 0xff) as u8,
    	((x >> 16) & 0xff) as u8,
    	((x >> 24) & 0xff) as u8
	]
}

#[cfg(target_endian="big")]
pub fn i64_to_bytes(x: i64) -> [u8; 8]
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
pub fn i64_to_bytes(x: i64) -> [u8; 8]
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