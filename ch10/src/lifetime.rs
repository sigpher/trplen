use std::cmp::max;

pub fn max_of_refs<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if *a > *b { a } else { b }
}
