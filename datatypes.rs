fn main() {
    // Statically-typed language: it must know type of all variables at compile time
    // 1. Scalar: integers, floating-point numbers, Booleans, and characters.
    // 2. Compound: tuples and arrays
    // boolean
    let t = true;
    println!("{}",t);
    let f: bool = false;
    println!("{}",f);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; //destructuring

    println!("The value of y is: {}", y);
    // tuple dot-notation
    println!("The value of y is: {}", tup.1);

    let tup1 = (500, 6.4, 1);
    println!("The value of y is: {}", tup1.1);

    // array: arrays in Rust have a fixed length
    //let a: [i32;2] = [1]; error - 
    let a: [i32;1] = [1];
    let a = [1];
    println!("{}",a[0])
}
