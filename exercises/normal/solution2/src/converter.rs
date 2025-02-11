pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let (str, base) = get_input(num_str);
    let sum = process(str, base);

    // println!( "         sum {}",sum);
    out(sum, to_base)
}

fn get_input(num_str: &str) -> (&str, u32) {
    let a: Vec<&str> = num_str.split('(').collect();
    let str: Vec<_> = a[1].split(')').map(|s| s.trim().to_string()).collect();
    let base = str[0].parse::<u32>().unwrap();

    (a[0], base)
}

fn process(str: &str, base: u32) -> u32 {
    let mut sum  = 0;
    for i in str.chars() {
        match i {
            '0' => {
                sum = sum * base + 0;
            }
            '1' => {
                sum = sum * base + 1;
            }
            '2' => {
                sum = sum * base + 2;
            }
            '3' => {
                sum = sum * base + 3;
            }
            '4' => {
                sum = sum * base + 4;
            }
            '5' => {
                sum = sum * base + 5;
            }
            '6' => {
                sum = sum * base + 6;
            }
            '7' => {
                sum = sum * base + 7;
            }
            '8' => {
                sum = sum * base + 8;
            }
            '9' => {
                sum = sum * base + 9;
            }
            'A' => {
                sum = sum * base + 10;
            }
            'B' => {
                sum = sum * base + 11;
            }
            'C' => {
                sum = sum * base + 12;
            }
            'D' => {
                sum = sum * base + 13;
            }
            'E' => {
                sum = sum * base + 14;
            }
            'F' => {
                sum = sum * base + 15;
            }
            _ => {}
        }
    }
    sum
}

fn out (mut sum:u32,to_base: u32) -> String{
    let mut string = String::new();
    while sum!=0 {
        let ch = sum% to_base;
        let ch = match ch {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            10 => 'a',
            11 => 'b',
            12 => 'c',
            13 => 'd',
            14 => 'e',
            15 => 'f',
            _ => '0',
        };
        string.push(ch);
        sum /= to_base;
    }
    let s :String= string.chars().rev().collect();
    // println!("{}",s);
    s
}