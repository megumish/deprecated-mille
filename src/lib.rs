//! mille is toy vcs
#![deny(missing_docs)]

pub mod bool;
pub mod integer;

/// This is enable to take difference.
pub trait TakeDifference {
    /// difference of an object
    type Difference;

    /// take difference and return its difference
    fn take_difference(&self, another: &Self) -> Difference<Self::Difference>;
}

/// difference
/// これはトレイトにした方が良いと思うかもしれないが、とりあえずenumとして定義しておき
/// 複雑になってきてからトレイトにすることを考えるという決定をした。
#[derive(PartialEq, Debug)]
pub enum Difference<T> {
    /// no difference
    Same,
    /// some differences
    Diff(T),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_is_same() {
        struct Singleton;

        impl TakeDifference for Singleton {
            type Difference = ();
            fn take_difference(&self, _: &Self) -> Difference<Self::Difference> {
                Difference::Same
            }
        }

        let singleton1 = Singleton;
        let singleton2 = Singleton;

        assert_eq!(singleton1.take_difference(&singleton2), Difference::Same);
    }

}
