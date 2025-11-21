use std::io::{
    self,
    Read,
    Write,
};

use num_traits::{
    FromBytes,
    ToBytes,
};

pub trait WritePrim: Write {
    fn write_prim(&mut self, value: impl ToBytes) -> io::Result<()> {
        self.write_all(value.to_le_bytes().as_ref())?;
        Ok(())
    }
}

impl<T: Write> WritePrim for T {}

pub trait ReadPrim: Read {
    fn read_prim<const N: usize, U: FromBytes<Bytes = [u8; N]>>(&mut self) -> io::Result<U> {
        let mut buf = [0; N];
        self.read_exact(&mut buf)?;
        Ok(U::from_le_bytes(&buf))
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
