use std::io;

fn main(){

    let l0 = 'A'; //char

    let string: &str = "Fillipi"; //string literal: static

    let mut string2 = String::new();
    string2.push('F');
    string2.push_str(" Mangrich ");

    println!("{string2}");

    let mut string3 =  String::new();

    io::stdin()
        .read_line(&mut string3)
        .expect("Error");

    println!("Voce digitou: {}", string3);
    println!("Quantidade de letras: {}", string3.trim().len());
    println!("Quantidade de letras: {}", string3.trim().chars().count());

    println!("{}", string3.to_uppercase());
    
    println!("{}", string3.replace("a","b"));

    println!("{:-^30}", "exemplo")
}