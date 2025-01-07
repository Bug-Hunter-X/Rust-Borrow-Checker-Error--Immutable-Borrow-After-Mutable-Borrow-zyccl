fn main() {
    let mut x = 5;
    { // Create a new scope
        let y = &mut x; // y is a mutable reference to x
        *y += 1; // Modifies x through y
        println!("x = {}", x); // x is now 6
    } // Mutable reference goes out of scope

    let z = &x; // Now it is safe to create an immutable reference
    println!("z = {}", *z); // This will now print correctly
} 