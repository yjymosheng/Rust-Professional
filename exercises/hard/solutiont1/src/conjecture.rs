// 调试的println全被删掉了

pub fn goldbach_conjecture() -> String {
    let mut v = Vec::new();
    v.push(2usize);
    v.push(3usize);

    let mut cnt = 0;
    let mut sum = String::new();
let mut flag = true ;
    for i in 3.. {
        if is_prime(i) {
            v.push(i);
        } else if i % 2 == 1 {
            if manzu(&v, i) {
                continue;
            } else {
                // println!("i {} ", i);
                cnt += 1;
                if flag {

                sum += &format!("{}",i.to_string())
                }else {
                    sum += &format!(",{}",i.to_string())

                }
                flag= false;
            }
        }

        if cnt == 2 {
            break;
        }
    }
    sum
}

fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    // 可以优化，但没必要
    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// 满足的拼音 观察推导的公式，可以看出一定规律，我用纸币写在本上，但我描述不出来什么规律.....
fn manzu(v: &Vec<usize>, n: usize) -> bool {
    let mut bo = false;
    for &i in v.iter().rev() {
        let tmp = n - i;
        let a = tmp / 2;
        let b = (a as f64).sqrt() as usize;

        if bo == false && b * b == a {
            bo = true;
            break;
        }
    }

    bo
}
