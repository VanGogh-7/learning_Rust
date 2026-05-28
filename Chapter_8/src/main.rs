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
}
