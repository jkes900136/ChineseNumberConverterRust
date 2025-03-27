use std::io;

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

fn main() {
    println!("請輸入一個數字（0-999999999）：");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("讀取輸入失敗");

    match input.trim().parse::<u64>() {
        Ok(num) if num <= 999999999 => {
            let result = convert_to_chinese(num);
            println!("轉換結果：{}", result);
        },
        Ok(_) => println!("請輸入小於或等於999999999的數字"),
        Err(_) => println!("請輸入有效的數字"),
    }
}
