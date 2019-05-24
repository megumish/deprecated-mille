//! difference between bool values
use super::*;

/// left is difference to right
#[derive(Debug, PartialEq)]
pub enum LeftIs {
    /// Left is true, right is false
    True,
    /// Left is false, right is true
    False,
}

impl TakeDifference for bool {
    type Difference = LeftIs;

    fn take_difference(&self, another: &Self) -> Difference<Self::Difference> {
        if self == another {
            Difference::Same
        } else if self == &true {
            Difference::Diff(LeftIs::True)
        } else {
            Difference::Diff(LeftIs::False)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_bool() {
        assert_eq!(true.take_difference(&true), Difference::Same);
        assert_eq!(false.take_difference(&false), Difference::Same);
    }

    #[test]
    fn different_bool() {
        assert_eq!(true.take_difference(&false), Difference::Diff(LeftIs::True));
        assert_eq!(
            false.take_difference(&true),
            Difference::Diff(LeftIs::False)
        );
    }
}
