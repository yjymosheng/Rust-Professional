use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
let mut set : HashSet<&str> = input_str.split(",").map(|x| x.trim()).collect();
    set.len()
}