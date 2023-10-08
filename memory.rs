
static _A = 500; //static memory; know compilation size

fn main(){

    let _b = 5; //stack; know compilation size


    let undefined_size = somefunction(); //heap, unknow size; RAII
}