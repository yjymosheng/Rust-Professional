pub fn dp_rec_mc(amount: u32) -> u32 {
    process(amount)
}

const CASHS: [u32; 8] = [100, 50, 30, 20, 10, 5, 2, 1];

fn process(mut amount: u32) -> u32 {
    let mut a = 0;
    for i in CASHS {
        let n = amount / i;
        amount = amount % i;
        a += n;
    }
    a
}
