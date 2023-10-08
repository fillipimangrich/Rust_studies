

fn main(){
    //scalar literals
    let _unsigned_8_bits: u8 = 255;
    let _unsigned_16_bits: u16 = 1_000;
    let _unsigned_32_bits: u32 = 0xff;
    let _unsigned_64_bits: u64 = 0o77;
    let _unsigned_arch_bits: usize = 0b1111111;

    let _byte: u8 = b'A';

    let _signed_8_bits: i8 = 5;
    let _signed_16_bits: i16 = 5;
    let _signed_32_bits: i32 = 5;
    let _signed_64_bits: i64 = 5;
    let _signed_arch_bits: isize = 5;

    let _boolean: bool = true;

    let _float_8_bits: f32 = 2.5;
    let _float_16_bits: f64 = 51.6;

    let _character: char = 'A';

    //tuple
    let mut numbers: (i32,i32,i32) = (1,2,3);

    println!("{:?}", numbers);

    println!("The first element is: {:?}", numbers.0);

    let (a,_b,_c) = numbers;

    println!("The first element is: {:?}", a);    

    numbers.0 = 0;

    println!("The first element is: {:?}", numbers.0);

    //array

    let n: [i32;3] = [1,2,3];

    println!("{:?}",n[0]);

    
}