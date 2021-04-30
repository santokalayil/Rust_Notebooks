fn main() {
    let mut n = 0;
    loop {
        n += 1;
        if n < 10 {
            println!("The value of n is {}", n);
        } else {
            println!("The value of n is not below 10 now Therefore breaking... currentvalue is  {}", n);
            break;
        }
        
    }
    println!();
    println!("Going to a new loop .............................. to show continue.");
    n = 0;
    loop {
        n += 1;
        if n == 5 {
            println!(" CONTINUING WITHOUT PRINTING...");
            continue;
        } 
        else if n < 10 {
            println!("The value of n is {}", n);
        } else {
            println!("The value of n is not below 10 now Therefore breaking... currentvalue is  {}", n);
            break;
        }
        
    }
}
