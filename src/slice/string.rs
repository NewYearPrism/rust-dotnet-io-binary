use crate::slice::_7bit_code::split_7bc;

#[derive(Debug, thiserror::Error)]
pub enum SplitError {
    #[error("unable to split length of str")]
    Length(#[from] super::_7bit_code::SplitError),
    #[error("unexpected end of input")]
    End,
}

pub fn split_dotnet_str(buf: &[u8]) -> Result<(&[u8], &[u8]), SplitError> {
    let (len, buf): (u32, _) = split_7bc(buf)?;
    let a = buf.split_at_checked(len as _).ok_or(SplitError::End)?;
    Ok(a)
}
