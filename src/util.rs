use std::cmp::{min, max};

pub fn clamp(value: i32, min_value: i32, max_value: i32) -> i32 {
    max(min_value, min(max_value, value))
}

pub fn round_tie_up(value: f32) -> i32 {
    (value + 0.5).floor() as i32
}

pub fn round_tie_down(value: f32) -> i32 {
    (value - 0.5).ceil() as i32
}
