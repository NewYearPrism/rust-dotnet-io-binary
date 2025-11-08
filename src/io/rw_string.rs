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
        Read7BitCode,
        Write7BitCode,
    },
};

pub trait Write7BitCodeLengthString {
    fn write_7bit_code_length_string(&mut self, s: &str) -> io::Result<()>;
}

impl<T: Write7BitCode + Write> Write7BitCodeLengthString for T {
    fn write_7bit_code_length_string(&mut self, s: &str) -> io::Result<()> {
        self.write_7bit_code::<u32>(s.len().as_())?;
        self.write_all(s.as_ref())?;
        Ok(())
    }
}

#[derive(Debug, derive_more::From)]
pub enum ReadError {
    ReadLength(rw_7bit_code::ReadError),
    Io(io::Error),
}

pub trait Read7BitCodeLengthString {
    fn read_7bit_code_length_string<'a>(
        &mut self,
        f: impl FnOnce(u32) -> &'a mut String,
    ) -> Result<(), ReadError>;
}

impl<T: Read7BitCode + Read> Read7BitCodeLengthString for T {
    fn read_7bit_code_length_string<'a>(
        &mut self,
        f: impl FnOnce(u32) -> &'a mut String,
    ) -> Result<(), ReadError> {
        let length: u32 = self.read_7bit_code()?;
        let buf = f(length);
        let mut take = self.take(length as _);
        take.read_to_string(buf)?;
        Ok(())
    }
}
