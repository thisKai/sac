use std::{iter::Map, slice::IterMut};

#[doc(hidden)]
#[derive(Debug)]
pub struct Iter<'a, T: 'a>(
    Map<Map<IterMut<'a, Option<T>>, fn(&'a mut Option<T>) -> Option<T>>, fn(Option<T>) -> T>,
);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[doc(hidden)]
pub fn mut_options_slice_to_iterator<'a, T>(slice: &'a mut [Option<T>]) -> Iter<'a, T> {
    Iter(
        slice
            .iter_mut()
            .map(extract_owned_option as fn(&'a mut Option<T>) -> Option<T>)
            .map(extract_inner as fn(Option<T>) -> T),
    )
}

fn extract_owned_option<T>(item: &mut Option<T>) -> Option<T> {
    use std::mem;
    mem::replace(item, None)
}
fn extract_inner<T>(item: Option<T>) -> T {
    match item {
        Some(item) => item,
        None => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_an_iterator_that_collects() {
        let mut buf = vec![Some(String::from("blah")); 3];

        let slice = &mut buf[..];

        let iterator = mut_options_slice_to_iterator(slice);

        let collection: Vec<String> = iterator.collect();

        assert_eq!(collection, vec![String::from("blah"); 3]);
    }

    #[test]
    fn more_than_32_items() {
        let mut buf = vec![Some(String::from("blah")); 36];

        let slice = &mut buf[..];

        let iterator = mut_options_slice_to_iterator(slice);

        let collection: Vec<String> = iterator.collect();

        assert_eq!(collection, vec![String::from("blah"); 36]);
    }
}
