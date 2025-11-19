use num_traits::PrimInt;

use crate::_7bit_code::{
    _7BitDecode,
    DecodeState,
};

#[derive(Debug, derive_more::Display, derive_more::Error)]
pub enum SplitError {
    End,
    Overflow,
}

pub fn split_7bc<T: _7BitDecode + PrimInt>(buf: &[u8]) -> Result<(T, &[u8]), SplitError> {
    let mut builder = T::build_from_7bit_codes();
    let mut i = 0;
    loop {
        let &b = buf.get(i).ok_or(SplitError::End)?;
        match builder(b) {
            DecodeState::InProgress => (),
            DecodeState::Done(res) => break Ok((res, &buf[i..])),
            DecodeState::Overflow(_) => Err(SplitError::Overflow)?,
        }
        i += 1;
    }
}
