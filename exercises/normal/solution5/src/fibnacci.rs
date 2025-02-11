pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let vec = get_vec(threshold);
    vec.iter().filter(|&&num| num % 2 == 1).sum()
}

fn get_vec(threshold: u32) -> Vec<u32> {
    let mut vec = Vec::new();
    vec.push(0);
    vec.push(1);
    while vec[vec.len() - 1] < threshold {
        vec.push(vec[vec.len() - 2] + vec[vec.len() - 1]);
    }
    vec.pop();

    vec
}
