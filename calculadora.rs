use std::io;

fn main(){
    println!("{:-^30}", "Calculadora");

    let mesage = "Bem vindo!\n \
    Digite os numeros separados por virgula\n \
    exemplo: 1,2,3";

    println!("{mesage}");

    let mut s = String::new();

    io::stdin()
        .read_line(& mut s)
        .expect("Error");

    println!("{}", s.trim());

    let nums: Vec<i32> = s
        .split(",")
        .map(|c| c.trim().parse().expect("error")) 
        .collect();
    
    let result: i32 = nums.iter().sum();

    println!("o total e: {:?}", result);
}