use std::io;

mod ex1;
use  ex1::mensagem;
fn main() {
    
    println!("Digite um número inteiro:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler linha");

    let num: i32 = input.trim().parse().expect("Por favor, digite um número inteiro!");
    println!("Você digitou o número: {}", num);

    println!("Digite uma palavra:");
    input.clear(); // Limpa o conteúdo anterior da variável
    io::stdin().read_line(&mut input).expect("Falha ao ler linha");

    let palavra = input.trim();
    println!("Você digitou a palavra: {}", palavra);
    
    mensagem();
}