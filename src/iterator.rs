use super::*;

impl<I> TakeDifference for I
where
    I: Iterator,
{
    type Difference = ();

    fn take_difference(&self, another: &Self) -> Difference<Self::Difference> {
        Difference::Same
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_iterators_should_be_same() {
        let iter1 = Vec::new().iter();
        let iter2 = Vec::new().iter();

        assert_eq!(iter1.take_difference(&iter2), Difference::Same);
    }
}
