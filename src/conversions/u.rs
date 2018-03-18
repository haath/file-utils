

#[cfg(target_endian="big")]
pub fn bytes_to_u16(x: &[u8; 2]) -> u16
{
    ((x[0] as u16) <<  8) +
    ((x[1] as u16) <<  0)
}

#[cfg(target_endian="little")]
pub fn bytes_to_u16(x: &[u8; 2]) -> u16
{
    ((x[1] as u16) <<  8) +
    ((x[0] as u16) <<  0)
}

#[cfg(target_endian="big")]
pub fn bytes_to_u32(x: &[u8; 4]) -> u32
{
    ((x[0] as u32) << 24) +
    ((x[1] as u32) << 16) +
    ((x[2] as u32) <<  8) +
    ((x[3] as u32) <<  0)
}

#[cfg(target_endian="little")]
pub fn bytes_to_u32(x: &[u8; 4]) -> u32
{
    ((x[3] as u32) << 24) +
    ((x[2] as u32) << 16) +
    ((x[1] as u32) <<  8) +
    ((x[0] as u32) <<  0)
}

#[cfg(target_endian="big")]
pub fn bytes_to_u64(x: &[u8; 8]) -> u64
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
pub fn bytes_to_u64(x: &[u8; 8]) -> u64
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



#[cfg(target_endian="big")]
pub fn u16_to_bytes(x: u16) -> [u8; 2]
{
    [
    	((x >>  8) & 0xff) as u8,
    	((x >>  0) & 0xff) as u8
	]
}

#[cfg(target_endian="little")]
pub fn u16_to_bytes(x: u16) -> [u8; 2]
{
    [
		((x >>  0) & 0xff) as u8,
    	((x >>  8) & 0xff) as u8
	]
}

#[cfg(target_endian="big")]
pub fn u32_to_bytes(x: u32) -> [u8; 4]
{
    [
		((x >> 24) & 0xff) as u8,
    	((x >> 16) & 0xff) as u8,
    	((x >>  8) & 0xff) as u8,
    	((x >>  0) & 0xff) as u8
	]
}

#[cfg(target_endian="little")]
pub fn u32_to_bytes(x: u32) -> [u8; 4]
{
    [
		((x >>  0) & 0xff) as u8,
    	((x >>  8) & 0xff) as u8,
    	((x >> 16) & 0xff) as u8,
    	((x >> 24) & 0xff) as u8
	]
}

#[cfg(target_endian="big")]
pub fn u64_to_bytes(x: u64) -> [u8; 8]
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
pub fn u64_to_bytes(x: u64) -> [u8; 8]
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