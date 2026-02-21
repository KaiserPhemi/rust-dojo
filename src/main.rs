
fn main() {
    // Arrays
    let numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("Array of numbers: {:?}", numbers);
    println!("=======================");
    let fruits: [&str; 4] = ["apple", "banana", "orange", "mango"];
    println!("Array of fruits: {:?}", fruits);

    // Tuples
    let human= ("Phemi", 44, false);
    println!("Human: {:?}", human)

}
