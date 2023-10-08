fn say_hello(name: &String){
    println!("Hello {}!",name);
}

fn say_goodbye(name: &String){
    println!("Goodbye {}!",name);
}

fn main(){

    let a = String::from("Fillipi");
    let b = &a; //move ownership to b

    println!("O valor de a  e: {}", a);
    println!("O valor de b  e: {}", b);

    say_hello(&a);
    say_goodbye(&b);

}