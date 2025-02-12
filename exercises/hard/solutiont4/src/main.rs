
use std::time::{Duration, Instant};
use crate::calc_time::time_info;
//I AM NOT DONE 
//Calculated according to ISO8061 standard

mod calc_time;

const TEST_CASES: &[(&str, &str)] = &[
    ("2025-01-01", "1,3,1,364,27,0"),
    ("2025-01-18", "2,6,18,347,10,1"),
];

fn main() {
    for (input, expected) in TEST_CASES {
        let start = Instant::now();
        let result = time_info(*input);
        let duration = start.elapsed();


        if &result!=expected {
        println!("{result}");

        }
    }
}
