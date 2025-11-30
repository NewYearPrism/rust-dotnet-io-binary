use std::{
    any,
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

#[derive(Debug, thiserror::Error)]
#[error("‘unable to write 7-bit encoded `{prim_type_name}`")]
pub struct WriteError {
    #[source]
    source: io::Error,
    prim_type_name: &'static str,
}

pub trait Write7bc: Write {
    fn write_7bc(&mut self, value: impl _7BitEncode) -> Result<(), WriteError> {
        let bytes: ArrayVec<_, 16> = value.into_7bit_codes().collect();
        self.write_all(&bytes).map_err(|source| WriteError {
            source,
            prim_type_name: any::type_name_of_val(&bytes),
        })?;
        Ok(())
    }
}

impl<T: Write> Write7bc for T {}

#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error("‘unable to read 7-bit encoded `{prim_type_name}`")]
    Io {
        #[source]
        source: io::Error,
        prim_type_name: &'static str,
    },
    #[error("7-bit encoded `{0}` overflow")]
    DecodeOverflow(&'static str),
}

pub trait Read7bc: Read {
    fn read_7bc<T: _7BitDecode + PrimInt>(&mut self) -> Result<T, ReadError> {
        let mut b = [0; 1];
        let mut builder = T::build_from_7bit_codes();
        loop {
            self.read_exact(&mut b).map_err(|source| ReadError::Io {
                source,
                prim_type_name: any::type_name::<T>(),
            })?;
            match builder(b[0]) {
                DecodeState::Done(res) => break Ok(res),
                DecodeState::Overflow(_) => Err(ReadError::DecodeOverflow(any::type_name::<T>()))?,
                _ => {}
            }
        }
    }
}

impl<T: Read> Read7bc for T {}
