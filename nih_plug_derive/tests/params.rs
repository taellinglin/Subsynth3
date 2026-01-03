use nih_plug::prelude::*;

#[derive(Params)]
struct FlatParams {
    #[id = "one"]
    pub one: BoolParam,

    #[id = "two"]
    pub two: FloatParam,

    #[id = "three"]
    pub three: IntParam,
}

impl Default for FlatParams {
    fn default() -> Self {
        FlatParams {
            one: BoolParam::new("one", true),
            two: FloatParam::new("two", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
            three: IntParam::new("three", 0, IntRange::Linear { min: 0, max: 100 }),
        }
    }
}

#[derive(Params)]
struct GroupedParams {
    #[id = "one"]
    pub one: BoolParam,

    #[nested(group = "Some Group", id_prefix = "group1")]
    pub group1: FlatParams,

    #[id = "three"]
    pub three: IntParam,

    #[nested(group = "Another Group", id_prefix = "group2")]
    pub group2: FlatParams,
}

impl Default for GroupedParams {
    fn default() -> Self {
        GroupedParams {
            one: BoolParam::new("one", true),
            group1: FlatParams::default(),
            three: IntParam::new("three", 0, IntRange::Linear { min: 0, max: 100 }),
            group2: FlatParams::default(),
        }
    }
}

// This should result in the same `.param_map()` as `GroupedParams`
#[derive(Default, Params)]
struct PlainNestedParams {
    #[nested]
    pub inner: GroupedParams,
}

#[derive(Default, Params)]
struct GroupedGroupedParams {
    #[nested(group = "Top-level group")]
    pub one: GroupedParams,
}

#[derive(Params)]
struct NestedParams {
    #[id = "one"]
    pub one: BoolParam,

    #[nested(id_prefix = "two")]
    pub two: FlatParams,

    #[id = "three"]
    pub three: IntParam,
}

impl Default for NestedParams {
    fn default() -> Self {
        NestedParams {
            one: BoolParam::new("one", true),
            two: FlatParams::default(),
            three: IntParam::new("three", 0, IntRange::Linear { min: 0, max: 100 }),
        }
    }
}

#[derive(Params)]
struct NestedArrayParams {
    #[id = "one"]
    pub one: BoolParam,

    #[nested(array, group = "Nested Params")]
    pub lots_of_twos: [FlatParams; 3],

    #[id = "three"]
    pub three: IntParam,
}

impl Default for NestedArrayParams {
    fn default() -> Self {
        NestedArrayParams {
            one: BoolParam::new("one", true),
            lots_of_twos: [
                FlatParams::default(),
                FlatParams::default(),
                FlatParams::default(),
            ],
            three: IntParam::new("three", 0, IntRange::Linear { min: 0, max: 100 }),
        }
    }
}

mod param_order {
    use super::*;

    #[test]
    fn flat_params_maintain_definition_order() {
        let params = FlatParams::default();
        let param_ids: Vec<String> = params.param_map().into_iter().map(|(id, _, _)| id).collect();
        
        assert_eq!(param_ids.len(), 3, "Expected 3 parameters");
        assert_eq!(param_ids, vec!["one", "two", "three"]);
    }

    #[test]
    fn grouped_params_preserve_hierarchy() {
        let params = GroupedParams::default();
        let param_ids: Vec<String> = params.param_map().into_iter().map(|(id, _, _)| id).collect();
        
        let expected = vec![
            "one",
            "group1_one",
            "group1_two",
            "group1_three",
            "three",
            "group2_one",
            "group2_two",
            "group2_three",
        ];
        
        assert_eq!(param_ids.len(), expected.len());
        assert_eq!(param_ids, expected);
    }

    #[test]
    fn plain_nested_params_match_grouped_structure() {
        let plain_nested = PlainNestedParams::default();
        let grouped = GroupedParams::default();

        let plain_nested_mapping: Vec<(String, String)> = plain_nested
            .param_map()
            .into_iter()
            .map(|(id, _, group)| (id, group))
            .collect();
        
        let grouped_mapping: Vec<(String, String)> = grouped
            .param_map()
            .into_iter()
            .map(|(id, _, group)| (id, group))
            .collect();

        assert_eq!(plain_nested_mapping.len(), grouped_mapping.len());
        assert_eq!(plain_nested_mapping, grouped_mapping);
    }

    #[test]
    fn grouped_groups_maintain_flat_ids() {
        let params = GroupedGroupedParams::default();
        let param_ids: Vec<String> = params.param_map().into_iter().map(|(id, _, _)| id).collect();
        
        // No ID prefixes are applied at the top level
        let expected = vec![
            "one",
            "group1_one",
            "group1_two",
            "group1_three",
            "three",
            "group2_one",
            "group2_two",
            "group2_three",
        ];
        
        assert_eq!(param_ids, expected);
    }

    #[test]
    fn nested_params_preserve_position() {
        let params = NestedParams::default();
        let param_ids: Vec<String> = params.param_map().into_iter().map(|(id, _, _)| id).collect();

        // Nested parameters maintain their position without explicit grouping
        assert_eq!(
            param_ids,
            vec!["one", "two_one", "two_two", "two_three", "three"]
        );
    }

    #[test]
    fn nested_array_params_generate_indexed_ids() {
        let params = NestedArrayParams::default();
        let param_ids: Vec<String> = params.param_map().into_iter().map(|(id, _, _)| id).collect();
        
        // Array indices are appended to parameter IDs
        let expected = vec![
            "one", "one_1", "two_1", "three_1", 
            "one_2", "two_2", "three_2", 
            "one_3", "two_3", "three_3", 
            "three"
        ];
        
        assert_eq!(param_ids.len(), 11);
        assert_eq!(param_ids, expected);
    }
}

mod param_groups {
    use super::*;

    #[test]
    fn flat_params_have_no_groups() {
        let params = FlatParams::default();
        let param_groups: Vec<String> = params
            .param_map()
            .into_iter()
            .map(|(_, _, group)| group)
            .collect();
        
        // All flat parameters should have empty group strings
        assert_eq!(param_groups.len(), 3);
        assert!(param_groups.iter().all(|g| g.is_empty()));
    }

    #[test]
    fn grouped_params_assign_correct_groups() {
        let params = GroupedParams::default();
        let param_groups: Vec<String> = params
            .param_map()
            .into_iter()
            .map(|(_, _, group)| group)
            .collect();
        
        let expected = vec![
            "",
            "Some Group",
            "Some Group",
            "Some Group",
            "",
            "Another Group",
            "Another Group",
            "Another Group",
        ];
        
        assert_eq!(param_groups, expected);
    }

    #[test]
    fn grouped_groups_create_hierarchical_paths() {
        let params = GroupedGroupedParams::default();
        let param_groups: Vec<String> = params
            .param_map()
            .into_iter()
            .map(|(_, _, group)| group)
            .collect();
        
        let expected = vec![
            "Top-level group",
            "Top-level group/Some Group",
            "Top-level group/Some Group",
            "Top-level group/Some Group",
            "Top-level group",
            "Top-level group/Another Group",
            "Top-level group/Another Group",
            "Top-level group/Another Group",
        ];
        
        assert_eq!(param_groups, expected);
    }

    #[test]
    fn nested_params_without_explicit_groups() {
        let params = NestedParams::default();
        let param_groups: Vec<String> = params
            .param_map()
            .into_iter()
            .map(|(_, _, group)| group)
            .collect();
        
        // No explicit groups means all empty strings
        assert_eq!(param_groups.len(), 5);
        assert!(param_groups.iter().all(|g| g.is_empty()));
    }

    #[test]
    fn nested_array_params_add_numeric_suffixes() {
        let params = NestedArrayParams::default();
        let param_groups: Vec<String> = params
            .param_map()
            .into_iter()
            .map(|(_, _, group)| group)
            .collect();
        
        let expected = vec![
            "",
            "Nested Params 1",
            "Nested Params 1",
            "Nested Params 1",
            "Nested Params 2",
            "Nested Params 2",
            "Nested Params 2",
            "Nested Params 3",
            "Nested Params 3",
            "Nested Params 3",
            ""
        ];
        
        assert_eq!(param_groups.len(), 11);
        assert_eq!(param_groups, expected);
    }
}
