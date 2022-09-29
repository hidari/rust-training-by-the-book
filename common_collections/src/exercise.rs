use std::collections::HashMap;
use std::io;
use std::str::SplitWhitespace;

pub fn exercise1(){
    let numbers = vec![1, 2, 3, 5, 4, 4, 2, 2, 3, 5, 5, 5, 1];
    println!("orig: {:?}", numbers);

    // 平均値
    let mut mean = 0;
    let mut sum = 0;
    for num in &numbers {
        sum += num
    }
    mean = sum / numbers.len();

    // ソートされたときに中央にくる値
    let mut median = 0;
    // 直接numbersをimmutableにソートしたいがやり方が分からなかったので
    // 一旦to_vecで別の変数に取り出してからmutableなsort()を使う
    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort();
    // 半分の長さを取得しそのままindexとして使うことで奇数の場合は中央、
    // 偶数の場合は半分の一つ先をmedianとして得られる
    let center = sorted_numbers.len() / 2;
    println!("sorted: {:?} / center: {}", sorted_numbers, center);
    median = sorted_numbers[center];

    // 最頻値
    let mut mode: usize = 0;
    let mut map = HashMap::new();
    for number in &numbers {
        // or_insertは該当エントリーへの可変参照を返す
        let mut entry = map.entry(number).or_insert(0);
        *entry += 1
    }
    println!("count: {:?}", map);
    let mut sorted_map: Vec<_> = map.into_iter().collect();
    sorted_map.sort_by(|x, y| y.1.cmp(&x.1));
    println!("as tuple: {:?}", sorted_map);
    if let Some(item) = sorted_map.first() {
        mode = *item.0
    }

    // 結果表示
    println!("\nmean: {}\nmedian: {}\nmode: {}", mean, median, mode);
}

pub fn exercise2(){
    let example = String::from("star");

    if is_the_first_letter_vowel(&example) {
        println!("result: {}",example + "-hay");
        return;
    }

    // 最初の母音までの子音を取得する
    // ここでは文字列を伸長する必要があるためStringとしている
    let mut suffix = String::from("-");
    let mut index = 0;
    for char in example.chars() {
        if char == 'a' ||
            char == 'e' ||
            char == 'i' ||
            char == 'o' ||
            char == 'u' {
            break;
        } else {
            suffix.push(char);
            index += 1;
        }
    }

    // ここで結合できるためにsuffixはStringである必要があった
    // "-"の後ろに子音列があるので、そこに"ay"をくっつける。
    // 結合したものはStringにならざるを得ないので、後ほどprefixに
    // にくっつける際には&strに変えてやることになる
    let complete_suffix = suffix + "ay";
    println!("{}", complete_suffix);

    // suffixと結合するためにStringにする必要がある
    let prefix =String::from(&example[index..example.len()]);
    println!("{}", prefix);

    // Stringのprefixと&strのsuffixを結合する
    println!("{}", prefix + complete_suffix.as_str());

    return;
}

fn is_the_first_letter_vowel(target: &String) -> bool{
    // getは添字アクセスと同様に範囲で文字列スライス
    // (UTF-8エンコードされたバイト列)を取得する
    let first = target.get(0..1);
    match first {
        Some("a") => true,
        Some("e") => true,
        Some("i") => true,
        Some("o") => true,
        Some("u") => true,
        _ => false
    }
}

pub fn exercise3(){

    // commands:
    // add <name> to <department>
    // list <[department] | [all]>

    // data structure
    // [<department, name>]

    let mut company_directory:Vec<HashMap<String, String>> = vec![];


    let mut init1 = HashMap::new();
    init1.insert(String::from("department"), String::from("Engineering"));
    init1.insert(String::from("name"), String::from("Sally"));
    company_directory.push(init1);

    let mut init2 = HashMap::new();
    init2.insert(String::from("department"), String::from("Sales"));
    init2.insert(String::from("name"), String::from("Amir"));
    company_directory.push(init2);

    let mut init3 = HashMap::new();
    init3.insert(String::from("department"), String::from("Sales"));
    init3.insert(String::from("name"), String::from("Jeff"));
    company_directory.push(init3);

    loop {
        println!("Enter command: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().to_string();
        println!("command: {}", input);

        // 入力されたコマンドをパースする
        let mut operation = String::new();
        let mut is_name_part = false;
        let mut name = String::new();
        let mut is_department_part = false;
        let mut department = String::new();

        let split_input = input.split_whitespace();
        for word in split_input {
            if word == "add" {
                operation = word.to_string();
                is_name_part = true;
                continue;
            } else if word == "list" {
                operation = word.to_string();
                is_department_part = true;
                continue;
            }

            if operation != "add" && operation != "list" {
                println!("invalid command");
                break;
            }

            if word == "to" {
                is_name_part = false;
                is_department_part = true;
                continue;
            }

            if operation == "add" && is_name_part {
                if name.is_empty() {
                    name += word;
                } else {
                    name += " ";
                    name += word;
                }
                continue;
            }
            if operation == "add" || operation == "list" && is_department_part {
                if department.is_empty() {
                    department += word;
                } else {
                    department += " ";
                    department += word;
                }
                continue;
            }
        }
        println!("cmd: {}, name: {}, dep: {}", operation, name, department);

        if operation == "add" {
            // nameとdepartmentをハッシュマップとしてベクタに保存
            let mut employee = HashMap::new();
            employee.insert(String::from("department"), department);
            employee.insert(String::from("name"), name);
            company_directory.push(employee);
            continue;
        }

        // departmentに合わせて一覧表示
        if operation == "list" {

            let listing_target = &department;

            // 全体をソートする
            let mut result = company_directory.to_vec();
            result.sort_by(|a, b| a.get("department").cmp(&b.get("department")).then(a.get("name").cmp(&b.get("name"))));

            // all が指定された場合は全社員を表示する
            if listing_target == "all" {
                for employee in result {
                    display_line(&employee)
                }
                continue;
            }

            // 特定の部署が指定された場合はここでフィルターして表示する
            let filtered = result
                .iter()
                // ↓ここもっといい判定方法ないかな…
                .filter(|&employee| match employee.get("department") {
                    Some(department) => department == listing_target,
                    _ => false,
            });

            for employee in filtered {
                display_line(employee);
            }
            continue;
        }
    }
}

fn display_line(employee: &HashMap<String, String>){
    let mut display_department = String::new();
    if let Some(dep) = employee.get("department") {
        display_department = dep.to_string()
    }
    let mut display_name = String::new();
    if let Some(name) = employee.get("name") {
        display_name = name.to_string()
    }
    println!("department: {} / name: {}", display_department, display_name);
}
