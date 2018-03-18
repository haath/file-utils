
extern crate file_utils;

use file_utils::conversions::i::*;
use file_utils::conversions::u::*;

#[test]
pub fn b_to_s1()
{
	let b = [ 0x01, 0x80 ];
	let i = bytes_to_i16(&b);

	assert_eq!(-32767, i);
}

#[test]
pub fn b_to_s2()
{
	let b = [ 0xFF, 0xFF, 0xFF, 0xFF ];
	let u = bytes_to_u32(&b);
	let i = bytes_to_i32(&b);

	assert_eq!(4294967295, u);
	assert_eq!(-1, i);
}

#[test]
pub fn b_to_s3()
{
	let b = [ 0x0F, 0x0F, 0x0F, 0x0F ];
	let i = bytes_to_i32(&b);

	assert_eq!(252645135, i);
}

#[test]
pub fn b_to_f()
{
	let b = [  ]
}