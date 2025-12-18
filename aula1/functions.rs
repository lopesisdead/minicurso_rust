fn main() {
    let (a, b) = (100, 10);

    // Chamando uma função que retorna um booleano
    let checaMaior = checa_maior(a, b);
    println!("A é maior que B? {checaMaior}");
}

// Sintaxe: fn nome(parâmetro: tipo) -> tipo_retorno { corpo }
fn checa_maior(val1: i32, val2: i32) -> bool {
    let resultado = val1 > val2;
    
    // expressão final (sem ;) é o retorno
    resultado
}