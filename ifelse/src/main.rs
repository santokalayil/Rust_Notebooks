fn main() {
    let mut n = 23;
    println!("======== IF ==========");
    if n < 45 {
        println!("n is lesser than 45");
    }
    println!("======== IF ELSE ==========");
    n = 55;
    if n < 45 { // == != 
        println!("n is lesser than 45");
    } else {
        println!("n is not LEsser than 45");
    }

    println!("======== NESTED IF ELSE ==========");
    n = 45;
    if n < 45 { // == != etc.
        println!("n is lesser than 45");
    } else if n == 45 {
        println!("n is equal to  45");
    } else {
        println!("n is greater than 45");
    }
}
