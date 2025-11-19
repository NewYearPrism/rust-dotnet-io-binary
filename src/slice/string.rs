use crate::slice::_7bit_code::split_7bc;

#[derive(Debug, derive_more::From, derive_more::Display, derive_more::Error)]
pub enum SplitError {
    Length(super::_7bit_code::SplitError),
    End,
}

pub fn split_dotnet_str(buf: &[u8]) -> Result<(&[u8], &[u8]), SplitError> {
    let (len, buf): (u32, _) = split_7bc(buf)?;
    let a = buf.split_at_checked(len as _).ok_or(SplitError::End)?;
    Ok(a)
}
