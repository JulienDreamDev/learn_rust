const NAME: &str = "JOHN"; // CONST are alway IMMUTABLE and can be declared in any scope

fn main() {
    println!("Thy name is {NAME}");

    let tup: (i32, char, f64) = (54, 'o', 489.36); // Tupple of int, char and double
    let (x, y, z) = tup;
    println!("destrucuting the tupple {x} {y} {z}");
}
