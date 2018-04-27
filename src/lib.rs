#![feature(macro_at_most_once_rep)]

#[macro_export]
macro_rules! map {
    (
        $($key:tt : $value: expr),+
        $(,)?
    ) => {{
        let map_capacity = map!(@count $($key),*);

        let mut map_instance = ::std::collections::HashMap::with_capacity(map_capacity);

        $( map!{ @item map_instance, $key, $value } )+

        map_instance
    }};
    ( @item $map:ident, $key:expr, $value:expr ) => {
        $map.insert($key, $value);
    };
    ( @count_unit $($x:tt)* ) => { () };
    ( @count $($rest:expr),*) => {
        <[()]>::len(&[$(map!(@count_unit $rest)),*])
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
}
