use std::io;

fn main() {

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("falha ao ler linha");

    let n: i32 = n.trim().parse().expect("Nao é um numero");
    let fibonacci = fibo(n);

    println!("Fibonacci de {n}: {fibonacci}");
}


fn fibo(x: i32) -> i64{
    let mut nminus2:i64 = 1;
    let mut nminus1:i64 = 1;
    let mut n:i64 = 0;
    if x == 0 || x == 1{
        1
    } else{
    for _ in 2..x{
        n = nminus1 + nminus2;
        nminus2 = nminus1;
        nminus1 = n;
    }
    return n;
    }
}