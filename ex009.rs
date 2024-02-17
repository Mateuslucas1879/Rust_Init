use std::io;

fn main(){
    
    println!("Digite um nome:");
    let mut nome = String::new();

    
    io::stdin()
        .read_line(&mut nome)
        .expect("Failed to read line");

    let nome = nome.trim();
    if nome == "Luana"{
        println!("{nome}!!Por favor ir ao balcão")
    } else{
        println!("{nome}!!Favor esperar")
    }
}


