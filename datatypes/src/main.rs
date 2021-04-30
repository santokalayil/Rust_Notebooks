fn main() {
    let x: i32 = 78; // don't use mut if we are not changing it later.. it throws a warning
    println!("Datatype i32.. The value of x is {}", x);
    let y: i64 = 4567;
    println!("Datatype i64.. The value of y is {}", y);
}
