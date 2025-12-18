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

    let v = vec![1, 2, 3, 4, 5];
    let terceiro: &i32 = &v[2];
    println!("O terceiro elemento é {terceiro}");

    let terceiro: Option<i32> = v.get(2);
    match terceiro{
        Some(terceiro) => println!("O terceiro elemento é {terceiro}"),
        None => println!("Não há terceiro elemento"),
    }

}
