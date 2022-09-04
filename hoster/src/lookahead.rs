pub(crate) struct LookaheadIter<I>{
    pub(crate) iter: I,
}

impl<I> Iterator for LookaheadIter<I>
where I: Iterator {
    type Item = (I::Item, Option<I::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.iter.next()?;
        let second = self.iter.next();

        Some((first, second))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_iterates_multiple() {
        let items = vec![1, 2, 3];
        let mut iter = LookaheadIter{ iter: items.into_iter() };

        assert_eq!(iter.next(), Some((1, Some(2))));
        assert_eq!(iter.next(), Some((3, None)));
    }

    #[test]
    fn it_iterates_with_for() {
        let items = vec![1, 2, 3];
        let mut iter = LookaheadIter{ iter: items.into_iter() };
        let mut eq = vec![(1, Some(2)), (3, None)].into_iter();

        for (item, next) in &mut iter {
            assert_eq!(eq.next(), Some((item, next)));
        }

        assert_eq!(iter.next(), None);
    }
}
