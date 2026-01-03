use std::collections::BTreeMap;
use std::sync::Mutex;

use nih_plug::prelude::*;

#[derive(Params, Default)]
struct WrapperParams {
    #[nested(id_prefix = "foo")]
    pub inner: InnerParams,
}

#[derive(Params, Default)]
struct ArrayWrapperParams {
    #[nested(array)]
    pub inners: [InnerParams; 3],
}

#[derive(Default)]
struct InnerParams {
    /// The value `deserialize()` has been called with so we can check that the prefix has been
    /// stripped correctly.
    pub deserialize_called_with: Mutex<Option<BTreeMap<String, String>>>,
}

unsafe impl Params for InnerParams {
    fn param_map(&self) -> Vec<(String, ParamPtr, String)> {
        Vec::new()
    }

    fn serialize_fields(&self) -> BTreeMap<String, String> {
        // When nested in another struct, the ID prefix will be added to `bar`
        let mut data = BTreeMap::new();
        data.insert(String::from("bar"), String::from("baz"));

        data
    }

    fn deserialize_fields(&self, serialized: &BTreeMap<String, String>) {
        *self.deserialize_called_with.lock().unwrap() = Some(serialized.clone());
    }
}

mod persist {
    mod nested_prefix {

        use super::super::*;

        #[test]
        fn serialize_adds_id_prefix_to_nested_fields() {
            let params = WrapperParams::default();
            let serialized = params.serialize_fields();
            
            // The nested struct's "bar" key should be prefixed with "foo_"
            assert_eq!(serialized.len(), 1, "Expected exactly one serialized field");
            assert_eq!(
                serialized.get("foo_bar"),
                Some(&String::from("baz")),
                "Expected prefixed key 'foo_bar' with value 'baz'"
            );
        }

        #[test]
        fn deserialize_strips_prefix_before_passing_to_nested() {
            let mut serialized = BTreeMap::new();
            serialized.insert(String::from("foo_bar"), String::from("aaa"));

            let params = WrapperParams::default();
            params.deserialize_fields(&serialized);

            // The prefix should be stripped when passing to the inner struct
            let deserialized = params
                .inner
                .deserialize_called_with
                .lock()
                .unwrap()
                .take()
                .expect("deserialize should have been called");
            
            assert_eq!(deserialized.len(), 1);
            assert_eq!(
                deserialized.get("bar"),
                Some(&String::from("aaa")),
                "Inner struct should receive 'bar' (without prefix)"
            );
        }

        #[test]
        fn deserialize_filters_keys_without_matching_prefix() {
            let mut serialized = BTreeMap::new();
            serialized.insert(String::from("foo_bar"), String::from("aaa"));
            serialized.insert(
                String::from("something"),
                String::from("this should not be there"),
            );

            let params = WrapperParams::default();
            params.deserialize_fields(&serialized);

            // Only keys with the correct prefix should be passed to the nested struct
            let deserialized = params
                .inner
                .deserialize_called_with
                .lock()
                .unwrap()
                .take()
                .expect("deserialize should have been called");
            
            assert_eq!(
                deserialized.len(),
                1,
                "Only matching prefix keys should be passed"
            );
            assert_eq!(deserialized.get("bar"), Some(&String::from("aaa")));
            assert!(
                !deserialized.contains_key("something"),
                "Keys without prefix should be filtered out"
            );
        }
    }

    mod array_suffix {
        use super::super::*;

        #[test]
        fn serialize_adds_numeric_suffix_to_array_elements() {
            let params = ArrayWrapperParams::default();
            let serialized = params.serialize_fields();
            
            // Each array element should have its own numbered suffix
            assert_eq!(serialized.len(), 3, "Expected 3 serialized array elements");
            assert_eq!(
                serialized.get("bar_1"),
                Some(&String::from("baz")),
                "First array element should have '_1' suffix"
            );
            assert_eq!(
                serialized.get("bar_2"),
                Some(&String::from("baz")),
                "Second array element should have '_2' suffix"
            );
            assert_eq!(
                serialized.get("bar_3"),
                Some(&String::from("baz")),
                "Third array element should have '_3' suffix"
            );
        }

        #[test]
        fn deserialize_routes_suffixed_keys_to_correct_array_elements() {
            let mut serialized = BTreeMap::new();
            serialized.insert(String::from("bar_1"), String::from("aaa"));
            serialized.insert(String::from("bar_2"), String::from("bbb"));
            serialized.insert(String::from("bar_3"), String::from("ccc"));

            let params = ArrayWrapperParams::default();
            params.deserialize_fields(&serialized);
            
            // Each array element should receive its corresponding value
            for (idx, (inner, expected_value)) in params.inners.into_iter().zip(["aaa", "bbb", "ccc"]).enumerate() {
                let deserialized = inner
                    .deserialize_called_with
                    .lock()
                    .unwrap()
                    .take()
                    .expect(&format!("Array element {} should have received deserialize call", idx + 1));
                
                assert_eq!(
                    deserialized.len(),
                    1,
                    "Array element {} should have exactly one field",
                    idx + 1
                );
                assert_eq!(
                    deserialized.get("bar"),
                    Some(&String::from(expected_value)),
                    "Array element {} should receive value '{}' without suffix",
                    idx + 1,
                    expected_value
                );
            }
        }
    }
}
