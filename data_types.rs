/*
tipos primitivos de variáveis

bool - Boolean (true / false)

char - character

f32, f64 - 32-bits, 64-bits floats

i64, i32, i16, i8 - signed 16- ... integers
u64, u32, u16, u8 - unsigned 16-bits, ... integers

isize - pointer-sized signed integers
usize - pointer-sized unsigned integers
*/

fn main() {
    // inicializando e declarando variavel generica
    let variavel = "isso_eh_uma_variavel";

    // fazendo a uma variavel mutavel
    let mut variavel_mutavel = "essa var eh mutavel";

    // declarando multiplas variaveis
    let (nome, idade) = ("Matheus", 21);

    // constante global
    const CONSTANTE_GLOBAL: i64 = 9;
    // println!("Testando: {CONSTANTE_GLOBAL}");

    println!("O aluno {nome} tem {idade} anos");

    let mut array = [1, 2, 3];
    println!("{:?}", array);
    array[0] = array[1] + array[2];
    println!("{:?}", array);

    // shadowing:
    // uma variável imutável ainda pode assumir diferentes valores
    // desde que você nomeie uma variável com o mesmo nome de outra
    // que já foi declarada anteriormente

    let shadow = 8;
    println!("{}", shadow);
    let shadow = shadow + 1;
    println!("{}", shadow);

    {
        let shadow = shadow * shadow;
        println!("{}", shadow);

    }




}