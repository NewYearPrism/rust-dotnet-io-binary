use num_traits::{
    AsPrimitive,
    PrimInt,
    Unsigned,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum DecodeState<T> {
    #[default]
    InProgress,
    Done(T),
    Overflow(T),
}

pub trait _7BitDecode: Sized {
    fn build_from_7bit_codes() -> impl FnMut(u8) -> DecodeState<Self>;
}

impl<T: PrimInt + Unsigned + 'static> _7BitDecode for T
where
    u8: AsPrimitive<T>,
{
    fn build_from_7bit_codes() -> impl FnMut(u8) -> DecodeState<Self> {
        let mut res = T::zero();
        let mut offset = 0;
        let width = T::zero().count_zeros().as_();
        move |b| {
            if offset >= width {
                return DecodeState::Overflow(res);
            }
            res = res | ((b & 0x7F).as_() << offset);
            if b & 0x80 == 0 {
                return DecodeState::Done(res);
            }
            if offset + 7 >= width {
                return DecodeState::Overflow(res);
            }
            offset += 7;
            DecodeState::InProgress
        }
    }
}

pub trait _7BitEncode {
    fn into_7bit_codes(self) -> impl Iterator<Item = u8>;
}

struct Into7BitCodes<T>(Option<T>);

impl<T: PrimInt + Unsigned + AsPrimitive<u8>> Iterator for Into7BitCodes<T> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(s) = self.0.take() else { return None };
        let mut next = s.as_();
        if !(s >> 7).is_zero() {
            next |= 0x80;
            self.0.replace(s >> 7);
        }
        Some(next)
    }
}

impl<T: PrimInt + Unsigned + AsPrimitive<u8>> _7BitEncode for T {
    fn into_7bit_codes(self) -> impl Iterator<Item = u8> {
        Into7BitCodes(Some(self))
    }
}

#[cfg(test)]
mod tests {
    use crate::_7bit_code::{
        _7BitDecode,
        _7BitEncode,
        DecodeState,
    };

    #[test]
    fn test_encode_u32() {
        let a: u32 = 0b0011000_1100000_0110000_0000011;
        let mut codes = a.into_7bit_codes();
        assert_eq!(codes.next(), Some(0b_1_0000011));
        assert_eq!(codes.next(), Some(0b_1_0110000));
        assert_eq!(codes.next(), Some(0b_1_1100000));
        assert_eq!(codes.next(), Some(0b_0_0011000));
        assert_eq!(codes.next(), None);
    }

    #[test]
    fn test_encode_u64() {
        let a: u64 = 0b0000010_0000100_0001000_0011000_1100000_0110000_0000011;
        let mut codes = a.into_7bit_codes();
        assert_eq!(codes.next(), Some(0b_1_0000011));
        assert_eq!(codes.next(), Some(0b_1_0110000));
        assert_eq!(codes.next(), Some(0b_1_1100000));
        assert_eq!(codes.next(), Some(0b_1_0011000));
        assert_eq!(codes.next(), Some(0b_1_0001000));
        assert_eq!(codes.next(), Some(0b_1_0000100));
        assert_eq!(codes.next(), Some(0b_0_0000010));
        assert_eq!(codes.next(), None);
    }

    #[test]
    fn test_decode_u64_ok() {
        let codes = [
            0b10000010, 0b10000100, 0b10001000, 0b10011000, 0b11100000, 0b10110000, 0b00000011,
        ];
        let mut res = Default::default();
        let mut builder = u64::build_from_7bit_codes();
        for b in codes {
            res = builder(b);
        }
        assert_eq!(
            res,
            DecodeState::Done(0b0000011_0110000_1100000_0011000_0001000_0000100_0000010)
        )
    }

    #[test]
    fn test_decode_u64_overflow_just_in_time() {
        #[rustfmt::skip]
        let codes = [
            0b10000010, 0b10000100, 0b10001000,
            0b10011000, 0b11100000, 0b10110000,
            0b10000011, 0b10000011, 0b10000011,
            0b10000011,
        ];
        let mut builder = u64::build_from_7bit_codes();
        let mut res = Default::default();
        for b in codes {
            res = builder(b);
        }
        assert!(matches!(res, DecodeState::Overflow(_)));
    }
}
