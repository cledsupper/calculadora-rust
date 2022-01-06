use calculadora::Calculadora;
use std::io;

fn main() {
    println!("Calculadora simples em Rust [Teste auto-avaliativo]");
    println!("Eu cansei de corrigir bugs. Esta calculadora NÃO É estável!");
    println!("");
    println!("Instruções de uso");
    println!("1. Operadores suportados: + - / * ^ ( ) ");
    println!("2. Use = para terminar");
    println!("3. Caracteres suportados: alfanuméricos, operadores, quebra de linha");
    println!("4. \"3↵+↵(↵5↵*↵2↵)↵=↵\" é válido, \"3 + (5 * 2) =↵\" NÃO É!");
    println!("");
    
    let mut calc = Calculadora::nova(io::stdin());

    println!("{}", calc.sessao());
}
