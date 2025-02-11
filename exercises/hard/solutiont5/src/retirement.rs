// 参考资料 ： https://www.gov.cn/yaowen/liebiao/202409/content_6974294.htm

const MONTH_TABLE: [&str; 12] = [
    "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12",
];
pub fn retire_time(time: &str, tp: &str) -> String {
    let s: String;
    let base: usize;
    let mut yan_chi_month: i32 = 0;

    let mut _tuixiu_year: i32 = 0;
    // 判断性别类型，获得两种基础大类（男、女）
    if tp.contains("女职工") {
        // 女有两种情况，获得base值
        let a = tp.find('5').unwrap();
        base = match tp.get(a..a + 2) {
            Some(tmp) => tmp.parse::<usize>().expect("解析年龄错误"),
            None => {
                println!("没有指出年龄");
                0
            }
        };
    } else {
        base = 60;
    }
    let (year, mut month_index) = parse_time(time);

    // 目前有三类退休情况，通过获得的base值进行处理获取延迟的月份
    match base {
        60 => {
            yan_chi_month = if year >= 1978 {
                36
            } else if year < 1965 {
                0
            } else {
                (year - 1965) * 3
                    + match month_index {
                    0..=3 => 1,
                    4..=7 => 2,

                    8..=11 => 3,
                    _ => 0,
                }
            };
        }
        55 => {
            yan_chi_month = if year >= 1982 {
                36
            } else if year < 1970 {
                0
            } else {
                (year - 1970) * 3
                    + match month_index {
                    0..=3 => 1,
                    4..=7 => 2,

                    8..=11 => 3,
                    _ => 0,
                }
            };
        }
        50 => {
            yan_chi_month = if year >= 1985 {
                60
            } else if year < 1975 {
                0
            } else {
                (year - 1975) * 3
                    + match month_index {
                    0..=1 => 1,
                    2..=3 => 2,
                    4..=5 => 3,

                    6..=7 => 4,
                    8..=9 => 5,
                    10..=11 => 6,
                    _ => 0,
                }
            };
        }
        _ => {}
    }

    // 第一次处理获得小数点后的数据

    let res = base as f64 + (yan_chi_month as f64 / 12f64);
    let tuixiu_month = if res.fract() == 0.0 {
        format!("{}", res as i64)
    } else {
        format!("{:.2}", res)
    };

    // 退休年通过   基础年数(base) + 延迟月数的年数（yan_chi_month） + 可能存在的进位年数
    _tuixiu_year = year
        + base as i32
        + yan_chi_month / 12
        + if month_index + yan_chi_month % 12 >= 12 {
        1
    } else {
        0
    };
    month_index = (month_index + yan_chi_month) % 12;

    // format获得最终结果
    s = format!(
        "{},{},{}",
        format!("{}-{}", _tuixiu_year, MONTH_TABLE[month_index as usize]),
        tuixiu_month,
        yan_chi_month
    )
        .to_string();
    // }

    s
}

fn parse_time(time: &str) -> (i32, i32) {
    time.split_once('-')
        .map(|(a, b)| {
            (
                a.parse::<i32>().expect("no year"),
                b.parse::<i32>().expect("Invalid month_index format") - 1,
            )
        })
        .unwrap()
}
