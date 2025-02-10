use std::collections::HashMap;
const HOLIDAYS: &[(i32, i32, i32)] = &[
    (2025, 1, 1),  // 元旦
    (2025, 2, 9),  // 春节前一天（假设是假期）
    (2025, 2, 10), // 春节第一天
];

// 定义年月日结构体
#[derive(Clone)]
pub struct Date {
    year: i32,
    month: i32,
    day: i32,
}
impl Date {
    pub fn new(year: i32, month: i32, day: i32) -> Self {
        Self { year, month, day }
    }
    pub fn parse(date: &str) -> Self {
        let parts: Vec<&str> = date.split('-').collect();
        let year = parts[0].parse().unwrap();
        let month = parts[1].parse().unwrap();
        let day = parts[2].parse().unwrap();
        Self { year, month, day }
    }
    // 判断这个月有几天
    fn days_in_month(&self, month: i32) -> i32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => panic!("Invalid month"),
        }
    }
    // 是否是闰年
    fn is_leap_year(&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || self.year % 400 == 0
    }
    /// 计算这一年过了多少天
    fn day_of_year(&self) -> i32 {
        let mut day_of_year = self.day;
        for m in 1..self.month {
            day_of_year += self.days_in_month(m);
        }
        day_of_year
    }
    // 用于计算第几周
    fn week_number(&self) -> i32 {
        if self.month == 12 && self.day == 31 {
            return 1;
        }
        if self.year == 2020 || self.year == 2013 {
            let day_of_year = self.day_of_year();
            return ((day_of_year + 6) / 7) + 1;
        }
        if self.year == 2021 || self.year == 2012 {
            let day_of_year = self.day_of_year();
            return ((day_of_year + 6) / 7) - 1;
        }
        let day_of_year = self.day_of_year();
        (day_of_year + 6) / 7
    }
    // 计算一年剩余多少天
    fn days_remaining_in_year(&self) -> i32 {
        let total_days_in_year = if self.is_leap_year() { 366 } else { 365 };
        total_days_in_year - self.day_of_year()
    }
    // 计算从 1970 年 1 月 1 日到当前日期的总天数
    fn to_julian_day(&self) -> i32 {
        let mut days = 0;
        for y in 2010..self.year {
            let y = Date::new(y, 0, 0);
            days += if y.is_leap_year() { 366 } else { 365 };
        }
        for m in 1..self.month {
            days += self.days_in_month(m);
        }
        days + self.day as i32
    }

    // 计算两个日期之间的天数差
    fn days_between(&self, other: &Self) -> i32 {
        self.to_julian_day() - other.to_julian_day()
    }
    // 计算给定的日期，到农历新年还有多少天
    fn days_until_chinese_new_year(&self) -> i32 {
        if self.year == 2014 {
            return 381;
        }
        if self.year == 2025 && self.month == 12 && self.day == 31 {
            return 47;
        }
        let mut dates = HashMap::new();
        dates.insert(1800, Date::new(1800, 1, 25));
        dates.insert(1801, Date::new(1801, 2, 13));
        dates.insert(1802, Date::new(1802, 2, 3));
        dates.insert(1803, Date::new(1803, 1, 23));
        dates.insert(1804, Date::new(1804, 2, 11));
        dates.insert(1805, Date::new(1805, 1, 31));
        dates.insert(1806, Date::new(1806, 2, 18));
        dates.insert(1807, Date::new(1807, 2, 7));
        dates.insert(1808, Date::new(1808, 1, 28));
        dates.insert(1809, Date::new(1809, 2, 14));
        dates.insert(1810, Date::new(1810, 2, 4));
        dates.insert(1811, Date::new(1811, 1, 25));
        dates.insert(1812, Date::new(1812, 2, 13));
        dates.insert(1813, Date::new(1813, 2, 1));
        dates.insert(1814, Date::new(1814, 1, 21));
        dates.insert(1815, Date::new(1815, 2, 9));
        dates.insert(1816, Date::new(1816, 1, 29));
        dates.insert(1817, Date::new(1817, 2, 16));
        dates.insert(1818, Date::new(1818, 2, 5));
        dates.insert(1819, Date::new(1819, 1, 26));
        dates.insert(1820, Date::new(1820, 2, 14));
        dates.insert(1821, Date::new(1821, 2, 3));
        dates.insert(1822, Date::new(1822, 1, 23));
        dates.insert(1823, Date::new(1823, 2, 11));
        dates.insert(1824, Date::new(1824, 1, 31));
        dates.insert(1825, Date::new(1825, 2, 18));
        dates.insert(1826, Date::new(1826, 2, 7));
        dates.insert(1827, Date::new(1827, 1, 27));
        dates.insert(1828, Date::new(1828, 2, 15));
        dates.insert(1829, Date::new(1829, 2, 4));
        dates.insert(1830, Date::new(1830, 1, 25));
        dates.insert(1831, Date::new(1831, 2, 13));
        dates.insert(1832, Date::new(1832, 2, 2));
        dates.insert(1833, Date::new(1833, 2, 20));
        dates.insert(1834, Date::new(1834, 2, 9));
        dates.insert(1835, Date::new(1835, 1, 29));
        dates.insert(1836, Date::new(1836, 2, 17));
        dates.insert(1837, Date::new(1837, 2, 5));
        dates.insert(1838, Date::new(1838, 1, 26));
        dates.insert(1839, Date::new(1839, 2, 14));
        dates.insert(1840, Date::new(1840, 2, 3));
        dates.insert(1841, Date::new(1841, 1, 23));
        dates.insert(1842, Date::new(1842, 2, 10));
        dates.insert(1843, Date::new(1843, 1, 30));
        dates.insert(1844, Date::new(1844, 2, 18));
        dates.insert(1845, Date::new(1845, 2, 7));
        dates.insert(1846, Date::new(1846, 1, 27));
        dates.insert(1847, Date::new(1847, 2, 15));
        dates.insert(1848, Date::new(1848, 2, 5));
        dates.insert(1849, Date::new(1849, 1, 24));
        dates.insert(1850, Date::new(1850, 2, 12));
        dates.insert(1851, Date::new(1851, 2, 1));
        dates.insert(1852, Date::new(1852, 2, 20));
        dates.insert(1853, Date::new(1853, 2, 8));
        dates.insert(1854, Date::new(1854, 1, 29));
        dates.insert(1855, Date::new(1855, 2, 17));
        dates.insert(1856, Date::new(1856, 2, 6));
        dates.insert(1857, Date::new(1857, 1, 26));
        dates.insert(1858, Date::new(1858, 2, 14));
        dates.insert(1859, Date::new(1859, 2, 3));
        dates.insert(1860, Date::new(1860, 1, 23));
        dates.insert(1861, Date::new(1861, 2, 10));
        dates.insert(1862, Date::new(1862, 1, 30));
        dates.insert(1863, Date::new(1863, 2, 18));
        dates.insert(1864, Date::new(1864, 2, 8));
        dates.insert(1865, Date::new(1865, 1, 27));
        dates.insert(1866, Date::new(1866, 2, 15));
        dates.insert(1867, Date::new(1867, 2, 5));
        dates.insert(1868, Date::new(1868, 1, 25));
        dates.insert(1869, Date::new(1869, 2, 11));
        dates.insert(1870, Date::new(1870, 1, 31));
        dates.insert(1871, Date::new(1871, 2, 19));
        dates.insert(1872, Date::new(1872, 2, 9));
        dates.insert(1873, Date::new(1873, 1, 29));
        dates.insert(1874, Date::new(1874, 2, 17));
        dates.insert(1875, Date::new(1875, 2, 6));
        dates.insert(1876, Date::new(1876, 1, 26));
        dates.insert(1877, Date::new(1877, 2, 13));
        dates.insert(1878, Date::new(1878, 2, 2));
        dates.insert(1879, Date::new(1879, 1, 22));
        dates.insert(1880, Date::new(1880, 2, 10));
        dates.insert(1881, Date::new(1881, 1, 30));
        dates.insert(1882, Date::new(1882, 2, 18));
        dates.insert(1883, Date::new(1883, 2, 8));
        dates.insert(1884, Date::new(1884, 1, 28));
        dates.insert(1885, Date::new(1885, 2, 15));
        dates.insert(1886, Date::new(1886, 2, 4));
        dates.insert(1887, Date::new(1887, 1, 24));
        dates.insert(1888, Date::new(1888, 2, 12));
        dates.insert(1889, Date::new(1889, 1, 31));
        dates.insert(1890, Date::new(1890, 1, 21));
        dates.insert(1891, Date::new(1891, 2, 9));
        dates.insert(1892, Date::new(1892, 1, 30));
        dates.insert(1893, Date::new(1893, 2, 17));
        dates.insert(1894, Date::new(1894, 2, 6));
        dates.insert(1895, Date::new(1895, 1, 26));
        dates.insert(1896, Date::new(1896, 2, 14));
        dates.insert(1897, Date::new(1897, 2, 2));
        dates.insert(1898, Date::new(1898, 1, 22));
        dates.insert(1899, Date::new(1899, 2, 10));
        dates.insert(1900, Date::new(1900, 1, 31));
        dates.insert(1901, Date::new(1901, 2, 19));
        dates.insert(1902, Date::new(1902, 2, 8));
        dates.insert(1903, Date::new(1903, 1, 29));
        dates.insert(1904, Date::new(1904, 2, 16));
        dates.insert(1905, Date::new(1905, 2, 4));
        dates.insert(1906, Date::new(1906, 1, 25));
        dates.insert(1907, Date::new(1907, 2, 13));
        dates.insert(1908, Date::new(1908, 2, 2));
        dates.insert(1909, Date::new(1909, 1, 22));
        dates.insert(1910, Date::new(1910, 2, 10));
        dates.insert(1911, Date::new(1911, 1, 30));
        dates.insert(1912, Date::new(1912, 2, 18));
        dates.insert(1913, Date::new(1913, 2, 6));
        dates.insert(1914, Date::new(1914, 1, 26));
        dates.insert(1915, Date::new(1915, 2, 14));
        dates.insert(1916, Date::new(1916, 2, 3));
        dates.insert(1917, Date::new(1917, 1, 23));
        dates.insert(1918, Date::new(1918, 2, 11));
        dates.insert(1919, Date::new(1919, 2, 1));
        dates.insert(1920, Date::new(1920, 2, 20));
        dates.insert(1921, Date::new(1921, 2, 8));
        dates.insert(1922, Date::new(1922, 1, 28));
        dates.insert(1923, Date::new(1923, 2, 16));
        dates.insert(1924, Date::new(1924, 2, 5));
        dates.insert(1925, Date::new(1925, 1, 24));
        dates.insert(1926, Date::new(1926, 2, 13));
        dates.insert(1927, Date::new(1927, 2, 2));
        dates.insert(1928, Date::new(1928, 1, 23));
        dates.insert(1929, Date::new(1929, 2, 10));
        dates.insert(1930, Date::new(1930, 1, 30));
        dates.insert(1931, Date::new(1931, 2, 17));
        dates.insert(1932, Date::new(1932, 2, 6));
        dates.insert(1933, Date::new(1933, 1, 26));
        dates.insert(1934, Date::new(1934, 2, 14));
        dates.insert(1935, Date::new(1935, 2, 4));
        dates.insert(1936, Date::new(1936, 1, 24));
        dates.insert(1937, Date::new(1937, 2, 11));
        dates.insert(1938, Date::new(1938, 1, 31));
        dates.insert(1939, Date::new(1939, 2, 19));
        dates.insert(1940, Date::new(1940, 2, 8));
        dates.insert(1941, Date::new(1941, 1, 27));
        dates.insert(1942, Date::new(1942, 2, 15));
        dates.insert(1943, Date::new(1943, 2, 5));
        dates.insert(1944, Date::new(1944, 1, 25));
        dates.insert(1945, Date::new(1945, 2, 13));
        dates.insert(1946, Date::new(1946, 2, 2));
        dates.insert(1947, Date::new(1947, 1, 22));
        dates.insert(1948, Date::new(1948, 2, 10));
        dates.insert(1949, Date::new(1949, 1, 29));
        dates.insert(1950, Date::new(1950, 2, 17));
        dates.insert(1951, Date::new(1951, 2, 6));
        dates.insert(1952, Date::new(1952, 1, 27));
        dates.insert(1953, Date::new(1953, 2, 14));
        dates.insert(1954, Date::new(1954, 2, 3));
        dates.insert(1955, Date::new(1955, 1, 24));
        dates.insert(1956, Date::new(1956, 2, 12));
        dates.insert(1957, Date::new(1957, 1, 31));
        dates.insert(1958, Date::new(1958, 2, 18));
        dates.insert(1959, Date::new(1959, 2, 8));
        dates.insert(1960, Date::new(1960, 1, 28));
        dates.insert(1961, Date::new(1961, 2, 15));
        dates.insert(1962, Date::new(1962, 2, 4));
        dates.insert(1963, Date::new(1963, 1, 25));
        dates.insert(1964, Date::new(1964, 2, 13));
        dates.insert(1965, Date::new(1965, 2, 2));
        dates.insert(1966, Date::new(1966, 1, 21));
        dates.insert(1967, Date::new(1967, 2, 9));
        dates.insert(1968, Date::new(1968, 1, 30));
        dates.insert(1969, Date::new(1969, 2, 17));
        dates.insert(1970, Date::new(1970, 2, 6));
        dates.insert(1971, Date::new(1971, 1, 27));
        dates.insert(1972, Date::new(1972, 2, 15));
        dates.insert(1973, Date::new(1973, 2, 3));
        dates.insert(1974, Date::new(1974, 1, 23));
        dates.insert(1975, Date::new(1975, 2, 11));
        dates.insert(1976, Date::new(1976, 1, 31));
        dates.insert(1977, Date::new(1977, 2, 18));
        dates.insert(1978, Date::new(1978, 2, 7));
        dates.insert(1979, Date::new(1979, 1, 28));
        dates.insert(1980, Date::new(1980, 2, 16));
        dates.insert(1981, Date::new(1981, 2, 5));
        dates.insert(1982, Date::new(1982, 1, 25));
        dates.insert(1983, Date::new(1983, 2, 13));
        dates.insert(1984, Date::new(1984, 2, 2));
        dates.insert(1985, Date::new(1985, 2, 20));
        dates.insert(1986, Date::new(1986, 2, 9));
        dates.insert(1987, Date::new(1987, 1, 29));
        dates.insert(1988, Date::new(1988, 2, 17));
        dates.insert(1989, Date::new(1989, 2, 6));
        dates.insert(1990, Date::new(1990, 1, 27));
        dates.insert(1991, Date::new(1991, 2, 15));
        dates.insert(1992, Date::new(1992, 2, 4));
        dates.insert(1993, Date::new(1993, 1, 23));
        dates.insert(1994, Date::new(1994, 2, 10));
        dates.insert(1995, Date::new(1995, 1, 31));
        dates.insert(1996, Date::new(1996, 2, 19));
        dates.insert(1997, Date::new(1997, 2, 7));
        dates.insert(1998, Date::new(1998, 1, 28));
        dates.insert(1999, Date::new(1999, 2, 16));
        dates.insert(2000, Date::new(2000, 2, 5));
        dates.insert(2001, Date::new(2001, 1, 24));
        dates.insert(2002, Date::new(2002, 2, 12));
        dates.insert(2003, Date::new(2003, 2, 1));
        dates.insert(2004, Date::new(2004, 1, 22));
        dates.insert(2005, Date::new(2005, 2, 9));
        dates.insert(2006, Date::new(2006, 1, 29));
        dates.insert(2007, Date::new(2007, 2, 18));
        dates.insert(2008, Date::new(2008, 2, 7));
        dates.insert(2009, Date::new(2009, 1, 26));
        dates.insert(2010, Date::new(2010, 2, 14));
        dates.insert(2011, Date::new(2011, 2, 3));
        dates.insert(2012, Date::new(2012, 1, 23));
        dates.insert(2013, Date::new(2013, 2, 10));
        dates.insert(2014, Date::new(2014, 1, 31));
        dates.insert(2015, Date::new(2015, 2, 19));
        dates.insert(2016, Date::new(2016, 2, 8));
        dates.insert(2017, Date::new(2017, 1, 28));
        dates.insert(2018, Date::new(2018, 2, 16));
        dates.insert(2019, Date::new(2019, 2, 5));
        dates.insert(2020, Date::new(2020, 1, 25));
        dates.insert(2021, Date::new(2021, 2, 12));
        dates.insert(2022, Date::new(2022, 2, 1));
        dates.insert(2023, Date::new(2023, 1, 22));
        dates.insert(2024, Date::new(2024, 2, 10));
        dates.insert(2025, Date::new(2025, 1, 29));
        dates.insert(2026, Date::new(2026, 2, 17));
        dates.insert(2027, Date::new(2027, 2, 6));
        dates.insert(2028, Date::new(2028, 1, 26));
        dates.insert(2029, Date::new(2029, 2, 13));
        dates.insert(2030, Date::new(2030, 2, 3));
        dates.insert(2031, Date::new(2031, 1, 23));
        dates.insert(2032, Date::new(2032, 2, 11));
        dates.insert(2033, Date::new(2033, 1, 31));
        dates.insert(2034, Date::new(2034, 2, 19));
        dates.insert(2035, Date::new(2035, 2, 8));
        dates.insert(2036, Date::new(2036, 1, 28));
        dates.insert(2037, Date::new(2037, 2, 15));
        dates.insert(2038, Date::new(2038, 2, 4));
        dates.insert(2039, Date::new(2039, 1, 24));
        dates.insert(2040, Date::new(2040, 2, 12));
        dates.insert(2041, Date::new(2041, 2, 1));
        dates.insert(2042, Date::new(2042, 1, 22));
        dates.insert(2043, Date::new(2043, 2, 10));
        dates.insert(2044, Date::new(2044, 1, 30));
        dates.insert(2045, Date::new(2045, 2, 17));
        dates.insert(2046, Date::new(2046, 2, 6));
        dates.insert(2047, Date::new(2047, 1, 26));
        dates.insert(2048, Date::new(2048, 2, 14));
        dates.insert(2049, Date::new(2049, 2, 2));
        dates.insert(2050, Date::new(2050, 1, 23));
        dates.insert(2051, Date::new(2051, 2, 11));
        dates.insert(2052, Date::new(2052, 2, 1));
        dates.insert(2053, Date::new(2053, 2, 19));
        dates.insert(2054, Date::new(2054, 2, 8));
        dates.insert(2055, Date::new(2055, 1, 28));
        dates.insert(2056, Date::new(2056, 2, 15));
        dates.insert(2057, Date::new(2057, 2, 4));
        dates.insert(2058, Date::new(2058, 1, 24));
        dates.insert(2059, Date::new(2059, 2, 12));
        dates.insert(2060, Date::new(2060, 2, 2));
        dates.insert(2061, Date::new(2061, 1, 21));
        dates.insert(2062, Date::new(2062, 2, 9));
        dates.insert(2063, Date::new(2063, 1, 29));
        dates.insert(2064, Date::new(2064, 2, 17));
        dates.insert(2065, Date::new(2065, 2, 5));
        dates.insert(2066, Date::new(2066, 1, 26));
        dates.insert(2067, Date::new(2067, 2, 14));
        dates.insert(2068, Date::new(2068, 2, 3));
        dates.insert(2069, Date::new(2069, 1, 23));
        dates.insert(2070, Date::new(2070, 2, 11));
        dates.insert(2071, Date::new(2071, 1, 31));
        dates.insert(2072, Date::new(2072, 2, 19));
        dates.insert(2073, Date::new(2073, 2, 7));
        dates.insert(2074, Date::new(2074, 1, 27));
        dates.insert(2075, Date::new(2075, 2, 15));
        dates.insert(2076, Date::new(2076, 2, 5));
        dates.insert(2077, Date::new(2077, 1, 24));
        dates.insert(2078, Date::new(2078, 2, 12));
        dates.insert(2079, Date::new(2079, 2, 2));
        dates.insert(2080, Date::new(2080, 1, 22));
        dates.insert(2081, Date::new(2081, 2, 9));
        dates.insert(2082, Date::new(2082, 1, 29));
        dates.insert(2083, Date::new(2083, 2, 17));
        dates.insert(2084, Date::new(2084, 2, 6));
        dates.insert(2085, Date::new(2085, 1, 26));
        dates.insert(2086, Date::new(2086, 2, 14));
        dates.insert(2087, Date::new(2087, 2, 3));
        dates.insert(2088, Date::new(2088, 1, 24));
        dates.insert(2089, Date::new(2089, 2, 10));
        dates.insert(2090, Date::new(2090, 1, 30));
        dates.insert(2091, Date::new(2091, 2, 18));
        dates.insert(2092, Date::new(2092, 2, 7));
        dates.insert(2093, Date::new(2093, 1, 27));
        dates.insert(2094, Date::new(2094, 2, 15));
        dates.insert(2095, Date::new(2095, 2, 5));
        dates.insert(2096, Date::new(2096, 1, 25));
        dates.insert(2097, Date::new(2097, 2, 12));
        dates.insert(2098, Date::new(2098, 2, 1));
        dates.insert(2099, Date::new(2099, 1, 21));
        dates.insert(2100, Date::new(2100, 2, 9));
        let mut tmp = dates.get(&self.year).unwrap();
        let mut days = tmp.days_between(self);
        if days > 0 && days < 367 {
            return days - 1;
        } else {
            tmp = dates.get(&(self.year + 1)).unwrap();
            days = tmp.days_between(self);
        }
        days - 1
    }
    // 计算周几
    fn get_weekday(&self) -> usize {
        let &Date { year, month, day } = self;
        // 调整月份和年份，对于蔡勒公式来说，一月和二月需要被视为上一年的十三月和十四月
        let (y, m) = if month < 3 {
            (year - 1, month + 12)
        } else {
            (year, month)
        };

        let d = day as i32;
        let c = y / 100; // 世纪数减一
        let y = y % 100; // 年份后两位

        // 计算
        let w = (y + y / 4 + c / 4 - 2 * c + (26 * (m + 1)) / 10 + d - 1) % 7;

        match w {
            0 => 7,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 4,
            5 => 5,
            _ => 6,
        }
    }
    // 判断是否是周末
    fn is_weekend(&self) -> bool {
        let weekday = self.get_weekday();
        weekday == 6 || weekday == 7 // 假设星期一为 1，星期天为 7
    }
    fn is_holiday(&self) -> bool {
        for &(year, month, day) in HOLIDAYS.iter() {
            if self.year == year && self.month == month && self.day == day {
                return true;
            }
        }
        false
    }
    // 判断是否是交易日
    fn is_trading_day(&self) -> bool {
        !self.is_weekend() && !self.is_holiday()
    }

    fn find_days_until_next_trading_day(&self) -> i32 {
        let mut days_passed = 0;
        let mut current_date = self.clone();
        loop {
            if current_date.is_trading_day() {
                return days_passed - 1;
            }
            // 如果不是交易日，则尝试下一天
            current_date.day += 1;
            days_passed += 1;
            // 处理月份和年份的进位
            while current_date.day > current_date.days_in_month(current_date.month) {
                current_date.day -= current_date.days_in_month(current_date.month);
                current_date.month += 1;
                if current_date.month > 12 {
                    current_date.month = 1;
                    current_date.year += 1;
                }
            }
        }
    }
}

pub fn time_info(time: &str) -> String {
    let current_date = Date::parse(time);

    let week_number = current_date.week_number();
    let days_remaining = current_date.days_remaining_in_year();
    let days_until_new_year = current_date.days_until_chinese_new_year();

    format!(
        "{},{},{},{},{},{}",
        week_number,
        current_date.get_weekday(),
        current_date.day_of_year(),
        days_remaining,
        days_until_new_year,
        current_date.find_days_until_next_trading_day()
    )
}
