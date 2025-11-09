use core::borrow::Borrow;
use std::io::{
    self,
    Read,
    Write,
};

use num_traits::{
    FromBytes,
    ToBytes,
    Zero,
};

pub trait WritePrim {
    fn write_prim(&mut self, value: impl ToBytes) -> io::Result<()>;
}

impl<T: Write> WritePrim for T {
    fn write_prim(&mut self, value: impl ToBytes) -> io::Result<()> {
        self.write_all(value.to_le_bytes().as_ref())?;
        Ok(())
    }
}

pub trait ReadPrim {
    fn read_prim<U: FromBytes + ToBytes + Zero>(&mut self) -> io::Result<U>
    where
        <U as ToBytes>::Bytes: Borrow<<U as FromBytes>::Bytes>;
}

impl<T: Read> ReadPrim for T {
    fn read_prim<U: FromBytes + ToBytes + Zero>(&mut self) -> io::Result<U>
    where
        <U as ToBytes>::Bytes: Borrow<<U as FromBytes>::Bytes>,
    {
        let mut buf = U::zero().to_le_bytes();
        self.read_exact(buf.as_mut())?;
        Ok(U::from_le_bytes(buf.borrow()))
    }
}

#[cfg(test)]
mod tests {
    use crate::io::rw_prim::ReadPrim;

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
