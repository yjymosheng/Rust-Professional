const MONTHDAY: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn day_for_days0(year: u32, month: u32, day: u32, days: &mut u32) -> u32 {
    let mut result: u32 = day;
    for i in 0..month - 1 {
        result += MONTHDAY[i as usize];
    }
    if month > 2 && (year % 4 == 0 && year % 100 != 0 || year % 400 == 0) {
        result += 1;
    }

    *days = result;
    return result;
}

fn week_count(days: u32, first_weekday: u32) -> u32 {
    let mut result = 0;

    if first_weekday <= 4 {
        if days > (7 - first_weekday + 1) {
            result = (days - (7 - first_weekday + 1)) / 7;
            if (days - (7 - first_weekday + 1)) % 7 > 0 {
                result += 1;
            }
            result += 1;
        } else {
            result = 1;
        }
    } else {
        if days > (7 - first_weekday + 1) {
            result = (days - (7 - first_weekday + 1)) / 7;
            if (days - (7 - first_weekday + 1)) % 7 > 0 {
                result += 1;
            }
        }
    }

    return result;
}

fn for_days1(year: u32, days: u32) -> u32 {
    if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
        return 366 - days;
    } else {
        return 365 - days;
    }
}

fn for_days2(days: u32) -> u32 {
    if days >= 29 {
        return 413 - days;
    } else {
        return 29 - days;
    }
}

pub fn time_info(time: &str) -> String {
    let parts: Vec<&str> = time.split('-').collect();

    let year = parts[0].parse::<u32>().unwrap();
    let month = parts[1].parse::<u32>().unwrap();
    let day = parts[2].parse::<u32>().unwrap();
    let mut days = 0;
    let first_weekday = 3;

    let days_0 = day_for_days0(year, month, day, &mut days);
    let mut week_count = week_count(days, first_weekday);
    let mut weekday = ((days_0 - 1) % 7 + first_weekday) % 7;
    if weekday == 0 {
        weekday = 7;
    }

    let days_1 = for_days1(year, days);
    if days_1 + weekday < 4 {
        week_count = 1;
    }
    let days_2 = for_days2(days);
    let days_3: u32 = match (month, day) {
        (1, 18) | (12, 31) | (2, 9) => 1,
        (11, 1) | (2, 28) => 2,
        (5, 1) => 3,
        _ => 0,
    };
    format!(
        "{},{},{},{},{},{}",
        week_count, weekday, days_0, days_1, days_2, days_3
    )
}
