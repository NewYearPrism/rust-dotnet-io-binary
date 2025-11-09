use std::{
    io,
    io::{
        Read,
        Write,
    },
};

use num_traits::AsPrimitive;

use crate::io::{
    rw_7bit_code,
    rw_7bit_code::{
        Read7bc,
        Write7bc,
    },
};

pub trait WriteDotnetStr {
    fn write_dotnet_str(&mut self, s: &str) -> io::Result<()>;
}

impl<T: Write7bc + Write> WriteDotnetStr for T {
    fn write_dotnet_str(&mut self, s: &str) -> io::Result<()> {
        self.write_7bc::<u32>(s.len().as_())?;
        self.write_all(s.as_ref())?;
        Ok(())
    }
}

#[derive(Debug, derive_more::From, derive_more::Display, derive_more::Error)]
pub enum ReadError {
    ReadLength(rw_7bit_code::ReadError),
    Io(io::Error),
}

pub trait ReadDotnetStr {
    fn read_dotnet_str<'a>(
        &mut self,
        f: impl FnOnce(u32) -> &'a mut [u8],
    ) -> Result<(), ReadError>;

    fn read_dotnet_str_to<'a>(
        &mut self,
        buf: &'a mut Vec<u8>,
    ) -> Result<(), ReadError> {
        self.read_dotnet_str(|l| {
            buf.clear();
            buf.resize(l as _, 0);
            buf.as_mut()
        })?;
        Ok(())
    }
}

impl<T: Read7bc + Read> ReadDotnetStr for T {
    fn read_dotnet_str<'a>(
        &mut self,
        f: impl FnOnce(u32) -> &'a mut [u8],
    ) -> Result<(), ReadError> {
        let length: u32 = self.read_7bc()?;
        let buf = f(length);
        let mut take = self.take(length as _);
        take.read_exact(buf)?;
        Ok(())
    }
}
