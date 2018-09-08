use std::{iter::FilterMap, slice::IterMut};

#[doc(hidden)]
#[derive(Debug)]
pub struct LazySac<'a, T: 'a>(FilterMap<IterMut<'a, Option<T>>, fn(&'a mut Option<T>) -> Option<T>>);

impl<'a, T> LazySac<'a, T> {
    pub fn from_slice(slice: &'a mut [Option<T>]) -> Self {
        LazySac(
            slice
                .iter_mut()
                .filter_map(|mut_option_ref| mut_option_ref.take()),
        )
    }
}

impl<'a, T> Iterator for LazySac<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_an_iterator_that_collects() {
        let mut buf = vec![Some(String::from("blah")); 3];

        let slice = &mut buf[..];

        let iterator = LazySac::from_slice(slice);

        let collection: Vec<String> = iterator.collect();

        assert_eq!(collection, vec![String::from("blah"); 3]);
    }

    #[test]
    fn more_than_32_items() {
        let mut buf = vec![Some(String::from("blah")); 36];

        let slice = &mut buf[..];

        let iterator = LazySac::from_slice(slice);

        let collection: Vec<String> = iterator.collect();

        assert_eq!(collection, vec![String::from("blah"); 36]);
    }
}
