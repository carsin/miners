use std::cmp::{min, max};

pub fn clamp(value: usize, min_value: usize, max_value: usize) -> usize {
    max(min_value, min(max_value, value))
}
