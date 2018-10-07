fn main() {
    // mut key word to make variable mutable. Variables are immutable by default
    let mut x = 10;
    println!("value of x:{}",x);
    x = 11;
    println!("value of x:{}", x);
    // constant 
    const MAX_POINTS: u32 = 100_000;    
    println!("Value of contast:{}",MAX_POINTS);

    //shadowing
    let y = 1;
    println!("Value of y:{}", y);
    //y = y + 1; errors!
    let y = y + 1;
    println!("Value of y:{}", y);
}