

fn say_hello(name: &str){
    println!("Hello {}!",name);
}

fn add(x:i32, y:i32) -> i32{
    x+y // the return are the last expression
}

fn main(){
    say_hello("Fillipi");

    let y = {
        say_hello("joao");
    };

    let input = "1 2 3 4 5 6 7 8 9";

    let result : Vec<i32> = input
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .map(|n| n*2)
        .collect();

    println!("{:?}", result);
}