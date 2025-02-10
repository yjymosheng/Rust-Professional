use std::collections::HashMap;

const TRIMONTHLY: [u32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn is_runyear(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn str_to_num(s: &str) -> u32 {
    let tmp = s.as_bytes();
    let mut number = 0;
    for num in tmp.iter() {
        number = number * 10 + (*num - b'0') as u32
    }
    number
}

fn week_day(year: u32, month: u32, day: u32) -> u32 {
    let mut ans = 0;
    let j = (year / 100) as i32;
    let k = (year % 100) as i32;
    let month = if month == 1 || month == 2 {
        month + 12
    } else {
        month
    };
    let h = (k + k/4 + j/4 - 2*j + (26*(month as i32+1)/10 + day as i32 - 1)) % 7;
    if (h+6) % 7 == 0 {
        ans = 7;
    } else {
        ans = (h as u32 + 6) % 7;
    }

    ans
}

fn week_count(year: u32, month: u32, day: u32) -> u32 {
    let week_day = week_day(year, 1, 1);
    let mut ans = (day + week_day - 1 ) / 7;
    if ans == 0 {
        ans = 1;
    }

    ans
}

fn days(year: u32, month: u32, day: u32) -> (u32, u32) {
    let mut ans_0 = 0;
    let mut ans_1 = 0;
    let is_leapyear = is_runyear(year);
    for m in 1..month {
        if is_leapyear && m == 2 {
            ans_0 += TRIMONTHLY[m as usize] + 1;
        } else {
            ans_0 += TRIMONTHLY[m as usize];
        }
    }
    ans_0 += day;

    if is_leapyear {
        ans_1 = 366 - ans_0;
    } else {
        ans_1 = 365 - ans_0;
    }

    (ans_0, ans_1)
}

fn compute_date(year0: u32, month0: u32, day0: u32, year1: u32, month1: u32, day1: u32) -> u32 {
    if year0 == year1 {
        let (days_0, _) = days(year0, month0, day0);
        let (days_1, _) = days(year1, month1, day1);
        days_0 - days_1 - 1
    } else {
        0
    }
}

fn newyear(year: u32, month: u32, day: u32, date2newyear: HashMap<u32, (u32, u32)>) -> u32 {
    let mut newyear_year = year;
    let mut newyear_month = 0;
    let mut newyear_day = 0;
    (newyear_month, newyear_day) = *date2newyear.get(&year).expect("date_to_newyear error");
    if month > newyear_month || (month == newyear_month && day > newyear_day) {
        (newyear_month, newyear_day) = *date2newyear.get(&(year+1)).expect("date_to_newyear error");
        newyear_year = year + 1;
    }
    compute_date(newyear_year, newyear_month, newyear_day,
                 year, month, day)
}

fn holiday(year: u32, month: u32, day: u32) -> u32 {

    let week_day = week_day(year, month, day);
    match week_day {
        1..=5 => 0,
        6 => 1,
        7 => 0,
        _ => panic!("date_to_trading_days error"),
    }
}

pub fn time_info(time: &str) -> String {
    let tmp: Vec<&str> = time.split('-').collect();
    let year = str_to_num(tmp[0]);
    let month = str_to_num(tmp[1]);
    let day = str_to_num(tmp[2]);

    let week_count = week_count(year, month, day);
    let week_day = week_day(year, month, day);

    let (days_0, days_1) = days(year, month, day);

    let mut date2newyear = HashMap::new();
    date2newyear.insert(2025, (1, 29));
    date2newyear.insert(2026, (1, 19));
    let days_2 = newyear(year, month, day, date2newyear);

    let days_3 = holiday(year, month, day);

    format!("{},{},{},{},{},{}", week_count, week_day, days_0, days_1, days_2, days_3)
}