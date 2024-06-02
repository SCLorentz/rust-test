use std::io; // imports
use rand::Rng;

fn main() { // função
    println!("heheboy"); // imprimir "heheboy" em uma nova linha

    println!("your number input:");
    let (mut input, mut y) = (String::new(), 10); // declarar variaveis tipo string e númerica. "mut" vem de 'mutavel'
    const VALUE: u32 = 15;
    let secret_number = rand::thread_rng().gen_range(1..=100); // gerar um número entre 1 e 100 e salvar em 'secret_number'

    io::stdin()
        .read_line(&mut input) // criar um input e salvar na variavel 'input'
        .expect("failed to read line"); // caso não seja possivel ler o input
    
    let _input: u32 = input // u32 se define como somente númerico
        .trim() // remove do input, elementos de whitespace como "/n" ou espaço
        .parse() // converte a string para um número
        .expect("please type a number!"); // o que será imprimido caso o valor de input não corresponder ao tipo solicitado
    
    println!("Your input was: {input}"); // imprimir o valor do input
    println!("10 + 15 = {}", y + VALUE); // imprimir uma variavel de outra forma (mais similar ao C)
    println!("the random number was: {secret_number}");

    loop {
        y += 1;
        print!("{y}");
        if y == 100 {
            print!("\n");
            break
        } else {
            print!("-");
        }
    }
}
