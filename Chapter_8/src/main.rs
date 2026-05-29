use std::collections::HashMap;
fn main() {
    let mut v0: Vec<i32> = Vec::new();
    v0.push(1);
    v0.push(2);
    v0.push(3);
    v0.push(4);

    let v1 = vec![1, 2, 3];
    let third: &i32 = &v1[2];
    println!("third: {}", third);
    let third: Option<&i32> = v1.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let mut v3 = vec![100, 20, 30];
    for i in &mut v3 {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let mut s = String::from("initial contents");
    s.push_str("bar");
    s.push('l');

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s_new = format!("{s1}-{s2}-{s3}");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    //adding a Key and Value Only if a Key Isn't present
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    //Updating a Value Based on the old Value

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");




}
