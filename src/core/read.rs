use std::io::{self, BufRead, ErrorKind, Seek, SeekFrom};

use byteorder::LittleEndian;

pub trait ReadExtTrait: BufRead + Seek {
    fn read_compressed_int(&mut self) -> io::Result<u32>;

    fn read_bool(&mut self) -> io::Result<bool>;

    fn read_u8(&mut self) -> io::Result<u8>;
    fn read_u16(&mut self) -> io::Result<u16>;
    fn read_i32(&mut self) -> io::Result<i32>;
    fn read_u32(&mut self) -> io::Result<u32>;

    fn read_f32(&mut self) -> io::Result<f32>;

    fn read_u24(&mut self) -> io::Result<u32>;

    fn read_bytes(&mut self, size: usize) -> io::Result<Vec<u8>>;

    fn read_string(&mut self, size: usize) -> io::Result<String>;

    fn read_string_lossy(&mut self, size: usize) -> io::Result<String>;
    fn read_string_zt(&mut self) -> io::Result<String>;
    fn peek_u8(&mut self) -> io::Result<u8>;

    fn peek_u16(&mut self) -> io::Result<u16>;

    fn peek_string(&mut self, size: usize) -> io::Result<String>;

    fn peek_string_lossy(&mut self, size: usize) -> io::Result<String>;
}

impl<T> ReadExtTrait for T
where
    T: BufRead + Seek,
{
    fn read_compressed_int(&mut self) -> io::Result<u32> {
        let val = ReadExtTrait::read_u8(self)?;
        let mut val = val;
        let mut ret = val as u32;
        while (val & 0x80) != 0 {
            val = ReadExtTrait::read_u8(self)?;
            dbg!(val);
            ret += (val.overflowing_sub(1).0).overflowing_mul(0x80).0 as u32;
        }

        Ok(ret)
    }

    fn read_bool(&mut self) -> io::Result<bool> {
        Ok(ReadExtTrait::read_u8(self)? != 0)
    }

    fn read_u8(&mut self) -> io::Result<u8> {
        byteorder::ReadBytesExt::read_u8(self)
    }

    fn read_u16(&mut self) -> io::Result<u16> {
        byteorder::ReadBytesExt::read_u16::<LittleEndian>(self)
    }

    fn read_i32(&mut self) -> io::Result<i32> {
        byteorder::ReadBytesExt::read_i32::<LittleEndian>(self)
    }

    fn read_u32(&mut self) -> io::Result<u32> {
        byteorder::ReadBytesExt::read_u32::<LittleEndian>(self)
    }

    fn read_f32(&mut self) -> io::Result<f32> {
        byteorder::ReadBytesExt::read_f32::<LittleEndian>(self)
    }

    fn read_u24(&mut self) -> io::Result<u32> {
        byteorder::ReadBytesExt::read_u24::<LittleEndian>(self)
    }

    fn read_bytes(&mut self, size: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; size];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_string(&mut self, size: usize) -> io::Result<String> {
        let buf = self.read_bytes(size)?;
        let str = String::from_utf8(buf);

        if let Ok(str) = str {
            Ok(str)
        } else {
            Err(io::Error::new(
                ErrorKind::InvalidData,
                "Invalid UTF8 String",
            ))
        }
    }

    fn read_string_lossy(&mut self, size: usize) -> io::Result<String> {
        let buf = self.read_bytes(size)?;
        let str = String::from_utf8_lossy(&buf);
        Ok(str.to_string())
    }

    fn read_string_zt(&mut self) -> io::Result<String> {
        let mut buf = Vec::new();
        self.read_until(b'\0', &mut buf)?;
        buf.pop();
        let str = String::from_utf8(buf);
        if let Ok(str) = str {
            Ok(str)
        } else {
            Err(io::Error::new(
                ErrorKind::InvalidData,
                "Invalid UTF-8 String",
            ))
        }
    }

    fn peek_u8(&mut self) -> io::Result<u8> {
        let pos = self.seek(SeekFrom::Current(0))?;
        let ret = ReadExtTrait::read_u8(self)?;
        self.seek(SeekFrom::Start(pos))?;
        Ok(ret)
    }

    fn peek_u16(&mut self) -> io::Result<u16> {
        let pos = self.seek(SeekFrom::Current(0))?;
        let ret = ReadExtTrait::read_u16(self)?;
        self.seek(SeekFrom::Start(pos))?;
        Ok(ret)
    }

    fn peek_string(&mut self, size: usize) -> io::Result<String> {
        let pos = self.seek(SeekFrom::Current(0))?;
        let ret = self.read_string(size)?;
        self.seek(SeekFrom::Start(pos))?;
        Ok(ret)
    }

    fn peek_string_lossy(&mut self, size: usize) -> io::Result<String> {
        let pos = self.seek(SeekFrom::Current(0))?;
        let ret = self.read_string_lossy(size)?;
        self.seek(SeekFrom::Start(pos))?;
        Ok(ret)
    }
}
