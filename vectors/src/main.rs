fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v1 = vec![1, 2, 3];

    v1.push(4);
    v1.push(5);
    v1.push(6);

    let third: &i32 = &v1[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v1.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v1[100];
    // let does_not_exist = v1.get(100);

    for i in &v1 {
        println!("{i}");
    }

    for i in &mut v1 {
        *i += 50;
        println!("{i}");
    }

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(1),
        SpreadSheetCell::Text(String::from("yellow")),
        SpreadSheetCell::Float(7.2),
    ];
}
