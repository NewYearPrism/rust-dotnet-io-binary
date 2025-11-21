use core::ops::DerefMut;
use std::io;

use crate::io::_7bit_code::{
    Read7bc,
    Write7bc,
};

pub trait WriteDotnetStr: Write7bc {
    fn write_dotnet_str(&mut self, s: &str) -> io::Result<()> {
        self.write_7bc(s.len() as u32)?;
        self.write_all(s.as_ref())?;
        Ok(())
    }
}

impl<T: Write7bc> WriteDotnetStr for T {}

#[derive(Debug, derive_more::From, derive_more::Display, derive_more::Error)]
pub enum ReadError {
    ReadLength(super::_7bit_code::ReadError),
    Io(io::Error),
}

pub trait ReadDotnetStr: Read7bc {
    fn read_dotnet_str<B: DerefMut<Target = [u8]>>(
        &mut self,
        f: impl FnOnce(u32) -> B,
    ) -> Result<B, ReadError> {
        use std::io::Read;
        let length: u32 = self.read_7bc()?;
        let mut buf = f(length);
        let mut take = self.take(length as _);
        take.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_dotnet_str_to<'a>(&mut self, buf: &'a mut Vec<u8>) -> Result<&'a mut [u8], ReadError> {
        let a = self.read_dotnet_str(|l| {
            buf.clear();
            buf.resize(l as _, 0);
            buf.as_mut_slice()
        })?;
        Ok(a)
    }
}

impl<T: Read7bc> ReadDotnetStr for T {}
