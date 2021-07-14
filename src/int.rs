use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Write};

const CODE_NEG_INT8: u8 = 0xff;
const CODE_INT16: u8 = 0xfe;
const CODE_INT32: u8 = 0xfd;
const CODE_INT64: u8 = 0xfc;

pub fn write_nat0<W: Write>(w: &mut W, v: u64) -> std::io::Result<()> {
    if v < 0x000000080 {
        w.write_all(&[v as u8])?;
    } else if v < 0x000010000 {
        w.write_all(&[CODE_INT16])?;
        w.write_all(&(v as u16).to_le_bytes())?;
    } else if v < 0x100000000 {
        w.write_all(&[CODE_INT32])?;
        w.write_all(&(v as u32).to_le_bytes())?;
    } else {
        w.write_all(&[CODE_INT64])?;
        w.write_all(&v.to_le_bytes())?;
    }
    Ok(())
}

pub fn write_i64<W: Write>(w: &mut W, v: i64) -> std::io::Result<()> {
    if 0 <= v {
        if v < 0x000000080 {
            w.write_all(&[v as u8])?;
        } else if v < 0x00008000 {
            w.write_all(&[CODE_INT16])?;
            w.write_all(&(v as u16).to_le_bytes())?;
        } else if v < 0x80000000 {
            w.write_all(&[CODE_INT32])?;
            w.write_all(&(v as u32).to_le_bytes())?;
        } else {
            w.write_all(&[CODE_INT64])?;
            w.write_all(&v.to_le_bytes())?;
        }
    } else if v >= -0x00000080 {
        w.write_all(&[CODE_NEG_INT8])?;
        w.write_all(&v.to_le_bytes()[..1])?;
    } else if v >= -0x00008000 {
        w.write_all(&[CODE_INT16])?;
        w.write_all(&v.to_le_bytes()[..2])?;
    } else if v >= -0x80000000 {
        w.write_all(&[CODE_INT32])?;
        w.write_all(&v.to_le_bytes()[..4])?;
    } else if v < -0x80000000 {
        w.write_all(&[CODE_INT64])?;
        w.write_all(&v.to_le_bytes())?;
    }
    Ok(())
}

pub fn read_signed<R: Read + ?Sized>(r: &mut R) -> std::io::Result<i64> {
    let c = r.read_u8()?;
    let v = match c {
        CODE_NEG_INT8 => r.read_i8()? as i64,
        CODE_INT16 => r.read_i16::<LittleEndian>()? as i64,
        CODE_INT32 => r.read_i32::<LittleEndian>()? as i64,
        CODE_INT64 => r.read_i64::<LittleEndian>()?,
        c => c as i64,
    };
    Ok(v)
}

pub fn read_nat0<R: Read + ?Sized>(r: &mut R) -> std::io::Result<u64> {
    let c = r.read_u8()?;
    let v = match c {
        CODE_INT16 => r.read_u16::<LittleEndian>()? as u64,
        CODE_INT32 => r.read_u32::<LittleEndian>()? as u64,
        CODE_INT64 => r.read_u64::<LittleEndian>()?,
        c => c as u64,
    };
    Ok(v)
}
