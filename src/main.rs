use std::io;
use std::collections::HashMap;

fn init_chinese_char_map() -> HashMap<char, u64> {
    let mut map = HashMap::new();
    map.insert('零', 0);
    map.insert('一', 1);
    map.insert('二', 2);
    map.insert('三', 3);
    map.insert('四', 4);
    map.insert('五', 5);
    map.insert('六', 6);
    map.insert('七', 7);
    map.insert('八', 8);
    map.insert('九', 9);
    map
}

fn init_unit_map() -> HashMap<char, u64> {
    let mut map = HashMap::new();
    map.insert('十', 10);
    map.insert('百', 100);
    map.insert('千', 1000);
    map.insert('万', 10000);
    map.insert('萬', 10000);
    map.insert('亿', 100000000);
    map.insert('億', 100000000);
    map
}

fn convert_to_chinese(num: u64) -> String {
    let chinese_digits = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];
    let units = ["", "十", "百", "千", "萬", "十", "百", "千", "億"];

    if num == 0 {
        return chinese_digits[0].to_string();
    }

    let mut result = String::new();
    let num_str = num.to_string();
    let mut skip_zero = true;
    let mut last_was_zero = false;

    for (i, c) in num_str.chars().enumerate() {
        let position = num_str.len() - 1 - i;
        let digit = c.to_digit(10).unwrap() as usize;

        if digit == 0 {
            if !skip_zero && !last_was_zero {
                result.push_str(chinese_digits[0]);
            }
            last_was_zero = true;
            continue;
        }

        last_was_zero = false;
        skip_zero = false;

        if digit != 1 || position != num_str.len() - 2 || num >= 100 {
            result.push_str(chinese_digits[digit]);
        }
        result.push_str(units[position]);
    }

    result
}

fn parse_mixed_number(input: &str) -> Result<u64, String> {
    let chinese_chars = init_chinese_char_map();
    let units = init_unit_map();
    
    let mut result: u64 = 0;
    let mut current_number: u64 = 0;
    let mut temp: u64 = 0;
    
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        let c = chars[i];
        
        if c.is_ascii_digit() {
            let mut num_str = String::new();
            while i < chars.len() && chars[i].is_ascii_digit() {
                num_str.push(chars[i]);
                i += 1;
            }
            temp = num_str.parse::<u64>().map_err(|_| "無效的數字格式")?;
            i -= 1;
        } else if let Some(&value) = chinese_chars.get(&c) {
            temp = value;
        } else if let Some(&unit) = units.get(&c) {
            if temp == 0 {
                temp = 1;
            }
            if unit >= 10000 {
                current_number = (current_number + temp) * unit;
                result += current_number;
                current_number = 0;
            } else {
                current_number += temp * unit;
            }
            temp = 0;
        } else if c.is_whitespace() || c == 'R' || c == 'M' || c == 'B' || c == '元' {
            // 忽略貨幣符號和空格
        } else {
            return Err(format!("無效的字符: {}", c));
        }
        i += 1;
    }
    
    result += current_number + temp;
    Ok(result)
}

fn main() {
    println!("請輸入數字（支援阿拉伯數字和中文混合寫法）：");
    println!("例如：4億2千萬、四亿2千500万、RMB 5億");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("讀取輸入失敗");

    match parse_mixed_number(input.trim()) {
        Ok(num) => {
            println!("數字值：{}", num);
            println!("中文表示：{}", convert_to_chinese(num));
        },
        Err(e) => println!("錯誤：{}", e),
    }
}
