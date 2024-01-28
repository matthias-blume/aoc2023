pub mod iter {

    pub trait BoxingIterator : Iterator {
        fn boxed(self) -> Box<[Self::Item]>;
    }

    impl<I> BoxingIterator for I where I: Iterator {
        fn boxed(self) -> Box<[Self::Item]> {
            self.collect::<Box<_>>()
        }
    }
}

/*
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
 */
