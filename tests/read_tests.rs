
extern crate file_utils;

use file_utils::read::Read;
use file_utils::write::Write;
use std::io::Cursor;

#[test]
fn reading()
{
	let mut c = Cursor::new(&[128, 3, 2, 1]);

	assert_eq!(16909184, c.read_u32().unwrap());

	c.set_position(0);

	assert_eq!(16909184, c.read_i32().unwrap());
}

#[test]
fn floating()
{
	let mut c = Cursor::new(&[128, 0, 0, 0]);

	assert_eq!(128.0, c.read_f32().unwrap());
}