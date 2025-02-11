pub fn new_birthday_probability(n: u32) -> f64 {
    process(n)
}

fn process(n: u32)->f64{
    let mut ans =1f64 ;
    if n>365 {
        ans=1f64;
    }
    for i in 0..n {
        ans *= (365 - i) as f64 / 365.0;
    }

    let ans = 1.0 - ans;
    ans
}