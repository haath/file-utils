#[cfg(target_endian="big")]
fn bytes_to_f32(x: &[u8; 4]) -> f32
{
    ((x[0] << 24) as f32) &
    ((x[1] << 16) as f32) &
    ((x[2] <<  8) as f32) &
    ((x[3] <<  0) as f32)
}

#[cfg(target_endian="little")]
fn bytes_to_f32(x: &[u8; 4]) -> f32
{
    ((x[3] << 24) as f32) ^
    ((x[2] << 16) as f32) ^
    ((x[1] <<  8) as f32) ^
    ((x[0] <<  0) as f32)
}

#[cfg(target_endian="big")]
fn bytes_to_f64(x: &[u8; 8]) -> f64
{	
    ((x[0] << 56) as f64) +
    ((x[1] << 48) as f64) +
    ((x[2] << 40) as f64) +
    ((x[3] << 32) as f64) +
    ((x[4] << 24) as f64) +
    ((x[5] << 16) as f64) +
    ((x[6] <<  8) as f64) +
    ((x[7] <<  0) as f64)
}

#[cfg(target_endian="little")]
fn bytes_to_f64(x: &[u8; 8]) -> f64
{
    ((x[7] << 56) as f64) +
    ((x[6] << 48) as f64) +
    ((x[5] << 40) as f64) +
    ((x[4] << 32) as f64) +
    ((x[3] << 24) as f64) +
    ((x[2] << 16) as f64) +
    ((x[1] <<  8) as f64) +
    ((x[0] <<  0) as f64)
}