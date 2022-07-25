const NAME: &str = "JOHN"; // CONST are alway IMMUTABLE and can be declared in any scope

fn main() {
    println!("Thy name is {NAME}");

    let tup: (i32, char, f64) = (54, 'o', 489.36); // Tupple of int, char and double
    let (x, y, z) = tup;
    println!("destructuring the tupple {x} {y} {z}");

    println!("3 squared is {}", squared(3));
}

fn squared(x: i32) -> i32 { // Can be defined after the main
    x * x // return value (without a semicolon)
}
