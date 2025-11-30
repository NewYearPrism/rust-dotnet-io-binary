use num_traits::FromBytes;

pub fn split_prim<const N: usize, T: FromBytes<Bytes = [u8; N]>>(buf: &[u8]) -> Option<(T, &[u8])> {
    let (bytes, buf1) = buf.split_first_chunk()?;
    Some((FromBytes::from_le_bytes(bytes), buf1))
}

#[cfg(test)]
mod tests {
    use crate::slice::prim::split_prim;

    #[test]
    fn test_split_prim() {
        let source = [0x01u8, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        let (int, tail): (u32, _) = split_prim(&source).unwrap();
        assert_eq!(int, 0x67452301);
        assert_eq!(tail, &[0x89, 0xab, 0xcd, 0xef]);
    }
}
