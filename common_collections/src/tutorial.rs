pub fn tutorial(){
    println!("Hello, world!");

    let mut v = vec![1,2,3,4,5];
    let third = &v[2];
    println!("The third elem is {}", third);

    match v.get(100) {
        Some(third) => println!("The third elem is {}.", third),
        None => println!("There is no third elem."),
    }
    // panicする
    // let over_run1 = &v[100];

    let _f1 = &v[0];
    // ここでvecの詰め替えがあった場合、f1にdanglingが発生している可能性がある
    // そのためf1はこの時点で使えなくなる
    v.push(6);
    // 新たに借用すると安全な参照が得られる
    let f2 = &v[0];

    // 一度可変借用を行ったあとは以前不変借用した参照は使えない
    // > cannot borrow `v` as mutable because it is also borrowed as immutable
    // というエラーは正しくないのでは？というのも
    //
    // let f1 = &v[0];
    // v.push(6);
    //
    // は実行できるじゃん？詳細メッセージの通り、変更後に参照しようとするのがまずいのでは？
    // エラーの通りだと↑に問題があるように感じる。でもホントはその後が問題だったという。
    // println!("The first elem is {}", f1);

    println!("The first elem is {}", f2);

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("value: {}", i)
    }

    // &を付けずに所有権を奪うとき、誰が所有権を持つことになるの？？
    // for i in v {
    //     println!("ok: {}", i)
    // }


    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let mut s = String::new();
    let data = "initial contents";
    s = data.to_string();
    s = String::from(data);
    s.push_str(" that modified");
    s.push('.');
    println!("{}", s);

    let hello = String::from("Hello, ");
    let world = String::from("World!");
    let result = hello + &world;
    println!("added: {}", result);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let formatted = format!("{}-{}-{}", tic,tac,toe);
    println!("formatted: {}", formatted);

    let greet = "Здравствуйте";
    let slice = &greet[0..4];
    println!("sliced: {}", slice);

    // 文字の境界ではないためpanicする
    // let error = &greet[0..1];
    // println!("error: {}", error);

    // chars()はUnicodeスカラ値
    for c in "नमस्ते".chars() {
        println!("char: {}",c);
    }

    // bytes()は各バイト（それぞれのバイトは必ずしもUnicodeスカラ値と一致しない）
    for b in "नमस्ते".bytes() {
        println!("byte: {}", b)
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![
        String::from("Blue"),
        String::from("Yellow")
    ];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams
        .iter()
        .zip(initial_scores.iter())
        .collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // 8.3 - ハッシュマップの値にアクセスする
    ////////////////////////////////////

    // 1. getによる個別アクセス
    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // getはOption型が返る

    // match式は最初のアームの型を返すことを期待するから
    // None => None,
    // みたいなのを一番最初に持ってくると本来書きたいprintln!が型違いでエラーになる
    match score {
        Some(i) => println!("{}", i),
        _ => ()
    };

    // if let気泡で書くとこう書ける
    // 今回の場合はNoneは無視していいのでこちらのほうが簡潔
    if let Some(num) = score {
        println!("{}", num)
    };

    // 2. forループによる走査
    for (key, value) in scores {
        println!("{}: {}", key, value)
    }


    // 8.3 - ハッシュマップを更新する
    /////////////////////////////

    // 値の上書き
    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Red"), 50);
    println!("{:?}", scores);

    // キーに値がなかった場合のみ挿入
    // entryはEntryというenumを返し、その値に応じてor_insertは値を挿入したり何もしなかったりする
    scores.entry(String::from("Blue")).or_insert(25);
    scores.entry(String::from("Red")).or_insert(15);
    // or_insertは値への可変参照を返すので適切に扱うことで値を上書きできたりする
    let mut ent = scores.entry(String::from("Green")).or_insert(50);
    *ent = *ent + 10; // Green: 60になる
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(String::from(word)).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
