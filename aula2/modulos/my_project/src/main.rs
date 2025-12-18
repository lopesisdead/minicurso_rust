use crate::jardim::vegetais::Aspargos;

pub mod jardim;

fn main() {
    let plant = Aspargos {};
    println!("Estou plantando {plant:?}!");
}