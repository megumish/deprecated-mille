//! difference between integers
use super::*;

/// sign of difference between inetger
#[derive(Debug, PartialEq)]
pub enum Sign {
    /// plus
    Plus,
    /// minus
    Minus,
}

macro_rules! unsigned_integer_take_difference {
    ($T:ty) => {
        impl TakeDifference for $T {
            type Difference = (Sign, $T);
            fn take_difference(&self, another: &Self) -> Difference<Self::Difference> {
                if self == another {
                    Difference::Same
                } else if self > another {
                    Difference::Diff((Sign::Plus, self - another))
                } else {
                    Difference::Diff((Sign::Minus, another - self))
                }
            }
        }
    };
}

unsigned_integer_take_difference!(u8);
unsigned_integer_take_difference!(u16);
unsigned_integer_take_difference!(u32);
unsigned_integer_take_difference!(u64);
unsigned_integer_take_difference!(u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_unsigned_integers() {
        assert_eq!(1u32.take_difference(&1u32), Difference::Same);
    }

    #[test]
    fn left_is_big_at_unsigned_integer() {
        assert_eq!(
            2u32.take_difference(&1u32),
            Difference::Diff((Sign::Plus, 1))
        );
    }

    #[test]
    fn left_is_small_at_unsigned_integer() {
        assert_eq!(
            1u32.take_difference(&2u32),
            Difference::Diff((Sign::Minus, 1))
        );
    }
}
