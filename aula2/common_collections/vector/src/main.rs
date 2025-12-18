fn main() {
    let v: Vec<i32> = Vec::new();

    //let v = vec![1, 2, 3];

    /*
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    */

    /*
    let v = vec![1, 2, 3, 4, 5];
    let terceiro: &i32 = &v[2];
    println!("O terceiro elemento é {terceiro}");

    let terceiro: Option<i32> = v.get(2);
    match terceiro{
        Some(terceiro) => println!("O terceiro elemento é {terceiro}"),
        None => println!("Não há terceiro elemento"),
    }
    */

    /*
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    */

    /* //Erro - Immutable borrow e mutable borrow
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");

     */

    /*

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

     */

    /*
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

     */

    /*
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

     */
}
