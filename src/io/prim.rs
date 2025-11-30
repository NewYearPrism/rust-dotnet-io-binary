use std::{
    any,
    io::{
        self,
        Read,
        Write,
    },
};

use num_traits::{
    FromBytes,
    ToBytes,
};

#[derive(Debug, thiserror::Error)]
#[error("unable to write `{prim_type_name}`")]
pub struct WriteError {
    #[source]
    source: io::Error,
    prim_type_name: &'static str,
}

pub trait WritePrim: Write {
    fn write_prim(&mut self, value: impl ToBytes) -> Result<(), WriteError> {
        self.write_all(value.to_le_bytes().as_ref())
            .map_err(|source| WriteError {
                source,
                prim_type_name: any::type_name_of_val(&value),
            })?;
        Ok(())
    }
}

impl<T: Write> WritePrim for T {}

#[derive(Debug, thiserror::Error)]
#[error("unable to read `{prim_type_name}`")]
pub struct ReadError {
    #[source]
    source: io::Error,
    prim_type_name: &'static str,
}

pub trait ReadPrim: Read {
    fn read_prim<const N: usize, T: FromBytes<Bytes = [u8; N]>>(&mut self) -> Result<T, ReadError> {
        let mut buf = [0; N];
        self.read_exact(&mut buf).map_err(|source| ReadError {
            source,
            prim_type_name: any::type_name::<T>(),
        })?;
        Ok(FromBytes::from_le_bytes(&buf))
    }
}

impl<T: Read> ReadPrim for T {}

#[cfg(test)]
mod tests {
    use crate::io::prim::ReadPrim;

    #[test]
    fn test_read_mixed() {
        let source = [0x12u8, 0x34, 0x56, 0x78, 0xAB, 0xCD, 0xEF, 0xFF];
        let mut s = source.as_slice();
        let a: i32 = s.read_prim().expect("should not fail");
        let mut s = source.as_slice();
        let b: i64 = s.read_prim().expect("should not fail");
        assert_eq!(a, 0x78563412);
        assert_eq!(b, 0xFFEFCDAB_78563412u64 as _);
    }
}
