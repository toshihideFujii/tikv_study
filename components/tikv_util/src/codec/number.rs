
use std::{
  io::{/*self, ErrorKind,*/ Write},
  mem
};

use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use super::{BytesSlice, Error, Result};

const SIGN_MARK: u64 = 0x8_000_000_000_000_000;
pub const MAX_VAR_I64_LEN: usize = 10;
pub const MAX_VAR_U64_LEN: usize = 10;
pub const U64_SIZE: usize = 8;
pub const I64_SIZE: usize = 8;
pub const F64_SIZE: usize = 8;

fn order_encode_i64(v: i64) -> u64 {
  v as u64 ^ SIGN_MARK
}

fn order_decode_i64(u: u64) -> i64 {
  (u ^ SIGN_MARK) as i64
}

fn order_encode_f64(v: f64) -> u64 {
  let u = v.to_bits();
  if v.is_sign_positive() {
    u | SIGN_MARK
  } else {
    !u
  }
}

fn order_decode_f64(u: u64) -> f64 {
  let u = if u & SIGN_MARK > 0 {
    u & (!SIGN_MARK)
  } else {
    !u
  };
  f64::from_bits(u)
}

pub trait NumberEncoder: Write {
  fn encode_i64(&mut self, v: i64) -> Result<()> {
    let u = order_encode_i64(v);
    self.encode_u64(u)
  }

  fn encode_i64_desc(&mut self, v: i64) -> Result<()> {
    let u = order_encode_i64(v);
    self.encode_u64_desc(u)
  }

  fn encode_u64(&mut self, v: u64) -> Result<()> {
    self.write_u64::<BigEndian>(v).map_err(From::from)
  }

  fn encode_u64_desc(&mut self, v: u64) -> Result<()> {
    self.write_u64::<BigEndian>(!v).map_err(From::from)
  }

  fn encode_u32(&mut self, v: u32) -> Result<()> {
    self.write_u32::<BigEndian>(v).map_err(From::from)
  }

  fn encode_u16(&mut self, v: u16) -> Result<()> {
    self.write_u16::<BigEndian>(v).map_err(From::from)
  }

  fn encode_var_i64(&mut self, v: i64) -> Result<()> {
    let mut vx = (v as u64) << 1;
    if v < 0 {
      vx = !vx;
    }
    self.encode_var_u64(vx)
  }

  fn encode_var_u64(&mut self, mut v: u64) -> Result<()> {
    while v >= 0x80 {
      self.write_u8(v as u8 | 0x80)?;
      v >>= 7;
    }
    self.write_u8(v as u8).map_err(From::from)
  }

  fn encode_f64(&mut self, f: f64) -> Result<()> {
    let u = order_encode_f64(f);
    self.encode_u64(u)
  }

  fn encode_f64_desc(&mut self, f: f64) -> Result<()> {
    let u = order_encode_f64(f);
    self.encode_u64_desc(u)
  }

  fn encode_u16_le(&mut self, v: u16) -> Result<()> {
    self.write_u16::<LittleEndian>(v).map_err(From::from)
  }

  fn encode_f32_le(&mut self, v: f32) -> Result<()> {
    self.write_f32::<LittleEndian>(v).map_err(From::from)
  }

  fn encode_u32_le(&mut self, v: u32) -> Result<()> {
    self.write_u32::<LittleEndian>(v).map_err(From::from)
  }

  fn encode_i32_le(&mut self, v: i32) -> Result<()> {
    self.write_i32::<LittleEndian>(v).map_err(From::from)
  }

  fn encode_f64_le(&mut self, v: f64) -> Result<()> {
    self.write_f64::<LittleEndian>(v).map_err(From::from)
  }

  fn encode_i64_le(&mut self, v: i64) -> Result<()> {
    self.write_i64::<LittleEndian>(v).map_err(From::from)
  }

  fn encode_u64_le(&mut self, v: u64) -> Result<()> {
    self.write_u64::<LittleEndian>(v).map_err(From::from)
  }
}

impl<T: Write> NumberEncoder for T {}


fn read_num_bytes<T, F>(size: usize, data: &mut &[u8], f: F) -> Result<T>
  where F: Fn(&[u8]) -> T
{
  if data.len() >= size {
    let buf = &data[..size];
    *data = &data[size..];
    return Ok(f(buf));
  }
  Err(Error::unexpected_eof())
}

#[inline]
pub fn decode_i64(data: &mut BytesSlice<'_>) -> Result<i64> {
  decode_u64(data).map(order_decode_i64)
}

#[inline]
pub fn decode_u64(data: &mut BytesSlice<'_>) -> Result<u64> {
  read_num_bytes(mem::size_of::<u64>(), data, BigEndian::read_u64)
}

#[inline]
pub fn decode_u32(data: &mut BytesSlice<'_>) -> Result<u32> {
  read_num_bytes(mem::size_of::<u32>(), data, BigEndian::read_u32)
}

#[inline]
pub fn decode_u16(data: &mut BytesSlice<'_>) -> Result<u16> {
  read_num_bytes(mem::size_of::<u16>(), data, BigEndian::read_u16)
}

#[inline]
pub fn decode_u64_desc(data: &mut BytesSlice<'_>) -> Result<u64> {
  let v = decode_u64(data)?;
  Ok(!v)
}

#[inline]
pub fn decode_f64(data: &mut BytesSlice<'_>) -> Result<f64> {
  decode_u64(data).map(order_decode_f64)
}

#[inline]
pub fn decode_f64_desc(data: &mut BytesSlice<'_>) -> Result<f64> {
  decode_u64_desc(data).map(order_decode_f64)
}

#[inline]
pub fn decode_u16_le(data: &mut BytesSlice<'_>) -> Result<u16> {
  read_num_bytes(mem::size_of::<u16>(), data, LittleEndian::read_u16)
}

#[inline]
pub fn decode_u32_le(data: &mut BytesSlice<'_>) -> Result<u32> {
  read_num_bytes(mem::size_of::<u32>(), data, LittleEndian::read_u32)
}

#[inline]
pub fn decode_i32_le(data: &mut BytesSlice<'_>) -> Result<i32> {
  read_num_bytes(mem::size_of::<i32>(), data, LittleEndian::read_i32)
}

#[inline]
pub fn decode_f64_le(data: &mut BytesSlice<'_>) -> Result<f64> {
  read_num_bytes(mem::size_of::<f64>(), data, LittleEndian::read_f64)
}

#[inline]
pub fn decode_f32_le(data: &mut BytesSlice<'_>) -> Result<f32> {
  read_num_bytes(mem::size_of::<f32>(), data, LittleEndian::read_f32)
}

#[inline]
pub fn decode_i64_le(data: &mut BytesSlice<'_>) -> Result<i64> {
  let v = decode_u64_le(data)?;
  Ok(v as i64)
}

#[inline]
pub fn decode_u64_le(data: &mut BytesSlice<'_>) -> Result<u64> {
  read_num_bytes(mem::size_of::<u64>(), data, LittleEndian::read_u64)
}