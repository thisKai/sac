#![feature(macro_at_most_once_rep)]

use std::iter::FromIterator;

pub fn mut_options_slice_to_collection<T, C: FromIterator<T>>(slice: &mut [Option<T>]) -> C {
    use std::mem;
    slice
        .iter_mut()
        .map(|item| mem::replace(item, None))
        .map(|item| item.unwrap())
        .collect()
}

#[macro_export]
macro_rules! map {
    (
        $($key:tt : $value: expr),+
        $(,)?
    ) => {{
        $crate::mut_options_slice_to_collection(&mut [
            $( Some(map!{ @item $key, $value }), )+
        ])
    }};
    ( @item $key:expr, $value:expr ) => {
        ($key, $value);
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn one_item_string_key() {
        let actual = map! {
            "key1": ()
        };
        let expected = {
            let mut expected = HashMap::with_capacity(1);
            expected.insert("key1", ());
            expected
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn one_item_string_key_trailing_comma() {
        let actual = map! {
            "key1": (),
        };
        let expected = {
            let mut expected = HashMap::with_capacity(1);
            expected.insert("key1", ());
            expected
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn many_items_string_key() {
        let actual = map! {
            "key1": (),
            "key2": (),
            "key3": (),
            "key4": ()
        };
        let expected = {
            let mut expected = HashMap::with_capacity(4);
            expected.insert("key1", ());
            expected.insert("key2", ());
            expected.insert("key3", ());
            expected.insert("key4", ());
            expected
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn many_items_string_key_trailing_comma() {
        let actual = map! {
            "key1": (),
            "key2": (),
            "key3": (),
            "key4": (),
        };
        let expected = {
            let mut expected = HashMap::with_capacity(4);
            expected.insert("key1", ());
            expected.insert("key2", ());
            expected.insert("key3", ());
            expected.insert("key4", ());
            expected
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn number_keys() {
        let actual = map! {
            0: (),
            1: (),
            2: (),
            3: (),
        };
        let expected = {
            let mut expected = HashMap::with_capacity(4);
            expected.insert(0, ());
            expected.insert(1, ());
            expected.insert(2, ());
            expected.insert(3, ());
            expected
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn non_copy_values() {
        #[derive(Debug, PartialEq)]
        struct NotCopy;

        let actual = map! {
            0: NotCopy,
        };
        let expected = {
            let mut expected = HashMap::with_capacity(4);
            expected.insert(0, NotCopy);
            expected
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn heap_allocates_string_values() {
        let actual = map! {
            0: "value".to_string(),
        };
        let expected = {
            let mut expected = HashMap::with_capacity(4);
            expected.insert(0, "value".to_string());
            expected
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn more_than_32_items() {
        let actual = map! {
            0: (),
            1: (),
            2: (),
            3: (),
            4: (),
            5: (),
            6: (),
            7: (),
            8: (),
            9: (),
            10: (),
            11: (),
            12: (),
            13: (),
            14: (),
            15: (),
            16: (),
            17: (),
            18: (),
            19: (),
            20: (),
            21: (),
            22: (),
            23: (),
            24: (),
            25: (),
            26: (),
            27: (),
            28: (),
            29: (),
            30: (),
            31: (),
            32: (),
            33: (),
            34: (),
            35: (),
            36: (),
        };
        let expected = {
            let mut expected = HashMap::with_capacity(4);
            expected.insert(0, ());
            expected.insert(1, ());
            expected.insert(2, ());
            expected.insert(3, ());
            expected.insert(4, ());
            expected.insert(5, ());
            expected.insert(6, ());
            expected.insert(7, ());
            expected.insert(8, ());
            expected.insert(9, ());
            expected.insert(10, ());
            expected.insert(11, ());
            expected.insert(12, ());
            expected.insert(13, ());
            expected.insert(14, ());
            expected.insert(15, ());
            expected.insert(16, ());
            expected.insert(17, ());
            expected.insert(18, ());
            expected.insert(19, ());
            expected.insert(20, ());
            expected.insert(21, ());
            expected.insert(22, ());
            expected.insert(23, ());
            expected.insert(24, ());
            expected.insert(25, ());
            expected.insert(26, ());
            expected.insert(27, ());
            expected.insert(28, ());
            expected.insert(29, ());
            expected.insert(30, ());
            expected.insert(31, ());
            expected.insert(32, ());
            expected.insert(33, ());
            expected.insert(34, ());
            expected.insert(35, ());
            expected.insert(36, ());
            expected
        };

        assert_eq!(expected, actual);
    }
}
