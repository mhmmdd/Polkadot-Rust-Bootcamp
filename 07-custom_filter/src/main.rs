struct FilterCondition {
    value: i32,
}

impl FilterCondition {
    fn is_match(&self, value: i32) -> bool {
        value == self.value
    }
}

fn custom_filter(values: Vec<i32>, condition: FilterCondition) -> Vec<i32> {
    let mut result = Vec::new();

    for value in values {
        if condition.is_match(value) {
            result.push(value);
        }
    }
    result
}

fn main() {
    let values = vec![1, 2, 3, 4, 5, 3, 5];
    let condition = FilterCondition { value: 3 };

    let result = custom_filter(values, condition);

    println!("Result: {:?}", result);
}

