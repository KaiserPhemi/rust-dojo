// Primitive  Types
// Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
// Floats: f32, f64
// u = unsigned, i = signed
// char = unicode character

// bool = true or false
// unit type = ()
// let x: u8 = 255; // overflow

// let x: u8 = 256; // overflow
// let x: u8 = 257; //

// Compund Types
// Tuple
// let tup: (i32, f64, u8) = (500, 6.4, 1);
// let (x, y, z) = tup;

// println!("The value of y is: {}", y);
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
