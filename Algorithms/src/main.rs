fn fib(n : u128, index : &mut [u128]) -> u128{
    if(index[n.into()] != 0){
        index[n.into()]
    }
    index[n.into()] = fib(n-1,  index) + fib(n-2, index);
    index[n.into()]
}
fn main() {
    let mut thing : [u128; 30];
    println!("{}", fib(30, &mut thing));
    println!("{:?}", thing);
}
