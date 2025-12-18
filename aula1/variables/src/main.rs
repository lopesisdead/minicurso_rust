fn main() {
   let x = 5;
   let x = x + 1;
   let spaces = "   ";
    let spaces = spaces.len();
    println!("space length = {spaces}");
   {
        let x = x * 2;
        println!("x = {x}");
   }
   println!("x = {x}");
}
