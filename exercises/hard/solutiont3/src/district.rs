use std::collections::{HashMap, HashSet};


pub fn count_provinces() -> String {
    let mut ans = String::new();
    let mut start = true;
    let file_content = std::fs::read_to_string("district.json").unwrap();
    let map = parse(file_content);
    let mut keys: Vec<_> = map.keys().collect();
    let mut cnt: u32;
    let mut flag: bool;
    keys.sort_by(|a, b| a.parse::<i32>().unwrap().cmp(&b.parse::<i32>().unwrap()));

    // 按顺序处理每个键
    for key in keys {
        let value = map.get(key).unwrap();
        // println!("Key: {}", key);
        cnt = 0;
        let mut visited = HashMap::new();
        for (inner_key, inner_set) in value.iter() {
            flag = true;
            // println!("  {} -> {:?}", inner_key, inner_set);
            if cnt == 0 {
                cnt += 1;
                visited.insert(inner_key, inner_set);
                continue;
            }
            // 如果我在别人的表里面 不应该+1
            else if visited.values().any(|set| set.contains(inner_key)) {
                flag = false;
                // println!("\tcnt : {}  , {:?}：我在别人的表里", cnt, inner_key);
                // continue;
            }
            //如果别人在我的表里 不应该+1
            else if visited
                .keys()
                .any(|bibao_key| inner_set.contains(*bibao_key))
            {
                flag = false;
                // println!("\tcnt : {}  , {:?}：别人在我的表里", cnt, inner_key);
                // continue;
            }
            // 我和别人的表里存在相同的市  不应该+1
            else if visited
                .iter()
                .any(|(tmp_key, tmp_set)| *tmp_key != inner_key && !inner_set.is_disjoint(tmp_set))
            {
                flag = false;
                // println!("\tcnt : {}  , {:?}：表里存在相同元素", cnt, inner_key);
                // continue;
            }
            if flag == true {
                cnt += 1;
            }
            visited.insert(inner_key, inner_set);
        }
        if start ==true {
            ans += &cnt.to_string();
            start = false;
        }else {
            ans +=",";
            ans += &cnt.to_string();

        }
    }
    ans
}

fn parse(file_content: String) -> HashMap<String, HashMap<String, HashSet<String>>> {
    let mut flag = false;
    // 手动解析 JSON 字符串
    // println!("{}",file_content);
    let mut total_hashmap: HashMap<String, HashMap<String, HashSet<String>>> = HashMap::new();
    let mut tmp_hashmap = HashMap::new();
    let mut number = String::new();
    for i in file_content.lines() {
        let line = i.trim();
        if line.contains(":") && line.starts_with("\"") && line.ends_with("{") {
            flag = true;
            tmp_hashmap.clear();
            number = (line.as_bytes()[1] - 48).to_string();
            // println!("number {}", number);
            // println!("\n\n\n");
            continue;
        } else if line.starts_with("\"") && (line.ends_with("],") || line.ends_with("]")) {
            // println!("{}", line);
            let tmp: Vec<_> = line.split(":").map(|s| s.to_string()).collect();
            // for j in &tmp{
            // println!("{}",tmp[0].trim().trim_matches('"').to_string());
            // tmp[1]  ["宜宾", "自贡", "绵阳", "泸州"],
            // {
            //     println!("\n\n{}",tmp[1]);
            //     let a = tmp[1].trim().trim_start_matches('[');
            //     println!("a {}\n\n",a);
            // }
            let city = tmp[0].trim().trim_matches('"').to_string();
            let city_table: HashSet<String> = tmp[1]
                .trim()
                .trim_start_matches("[")
                .trim_end_matches("],")
                .trim_end_matches(']')
                .split(",")
                .map(|s| s.trim().trim_matches(|c| c == '"').to_string())
                .collect();
            // println!("\n {:#?}",city_table);
            let mut merged = false;
            for (existing_city, existing_set) in tmp_hashmap.iter_mut() {
                if !city_table.is_disjoint(existing_set) || existing_city == &city {
                    // 合并当前城市到已有城市的表中
                    existing_set.insert(city.clone());
                    existing_set.extend(city_table.clone());
                    merged = true;
                    break;
                }
            }

            if !merged {
                // 如果没有找到交集，直接插入新的记录
                tmp_hashmap.insert(city, city_table);
            }
        // }
        } else if line == "}," || line == "}" {
            flag = false;
            total_hashmap.insert(number.clone(), tmp_hashmap.clone());
            // println!("\n\n\n");

            continue;
        }
    }
    // println!("{:#?}", total_hashmap);
    total_hashmap
}
