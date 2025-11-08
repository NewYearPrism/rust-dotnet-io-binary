use std::{
    io,
    io::{
        Read,
        Write,
    },
};

use arrayvec::ArrayVec;
use num_traits::PrimInt;

use crate::_7bit_code::{
    _7BitDecode,
    _7BitEncode,
    DecodeState,
};

pub trait Write7BitCode {
    fn write_7bit_code<U: _7BitEncode>(&mut self, value: U) -> io::Result<()>;
}

impl<T: Write> Write7BitCode for T {
    fn write_7bit_code<U: _7BitEncode>(&mut self, value: U) -> io::Result<()> {
        let bytes: ArrayVec<_, 16> = value.into_7bit_codes().collect();
        self.write_all(&bytes)?;
        Ok(())
    }
}

#[derive(Debug, derive_more::From)]
pub enum ReadError {
    Io(io::Error),
    #[from(skip)]
    DecodeOverflow,
}

pub trait Read7BitCode {
    fn read_7bit_code<U: _7BitDecode + PrimInt>(&mut self) -> Result<U, ReadError>;
}

impl<T: Read> Read7BitCode for T {
    fn read_7bit_code<U: _7BitDecode + PrimInt>(&mut self) -> Result<U, ReadError> {
        let mut b = [0; 1];
        let mut builder = U::build_from_7bit_codes();
        loop {
            self.read_exact(&mut b)?;
            match builder(b[0]) {
                DecodeState::Done(res) => break Ok(res),
                DecodeState::Overflow(_) => Err(ReadError::DecodeOverflow)?,
                _ => {}
            }
        }
    }
}
