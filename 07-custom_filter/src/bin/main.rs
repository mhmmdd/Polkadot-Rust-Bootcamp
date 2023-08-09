// Custom filter for every type
struct FilterCondition<T> {
    value: T,
}

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        *item == self.value
    }
}

fn custom_filter<T: PartialEq>(collection: Vec<T>, filter_condition: &FilterCondition<T>) -> Vec<T> {
    collection.into_iter().filter(|item| filter_condition.is_match(&item)).collect()
}

fn main() {
    let condition = FilterCondition { value: 42 };
    let items = vec![10, 42, 55, 42, 89];
    let filtered_items = custom_filter(items, &condition);
    println!("Filtered items: {:?}", filtered_items); // Output: Filtered items: [42, 42]

    let condition_str = FilterCondition { value: "apple".to_string() };
    let items_str = vec!["banana".to_string(), "apple".to_string(), "orange".to_string()];
    let filtered_items_str = custom_filter(items_str, &condition_str);
    println!("Filtered items: {:?}", filtered_items_str); // Output: Filtered items: ["apple"]
}
