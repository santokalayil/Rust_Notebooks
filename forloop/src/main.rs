fn main() {
    // for loop should use iterator.. it is a must
    for i in 1..11 {
        println!("The number is {}", i);
    }
    println!();
    println!("New Loop by assigning ");

    let numbers = 3..21; // this is range . range is an iterator
    for i in numbers {
        println!("THE NUMBER IS {}", i);
    }

    let mut animals = vec!["Rabbit", "Lion", "Rat", "Deer", "Elephant"];
    for animal in animals { // HERE the ownnershop of vector elements are moved to for loop
        println!("{}", animal);
    }
    
    println!();
    println!("HERE OWNERSHIP IS NOT MOVED TO FOR lOOP BCZ OF .iter() method..");
    animals = vec!["DOG","CAT","SQUIRREL"];
    for animal in animals.iter() { // HERE the ownnershop of vector elements are NOT moved to for loop since we use iter
        println!("{}", animal);
    }

    println!();
    println!("Enumerate with .iter() method.."); // Enumerate without .iter() method is NOT SUPPORTED..
    println!();
    animals = vec!["DOG","CAT","SQUIRREL"];
    for (idx, animal) in animals.iter().enumerate() { 
        println!("{} - {}", idx, animal);
    }


}
