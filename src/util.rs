use std::cmp::{min, max};

pub fn clamp(value: i32, min_value: i32, max_value: i32) -> i32 {
    max(min_value, min(max_value, value))
}
