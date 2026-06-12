fn main() {
    let mut output: i32 = 20;
    for _ in 0..1000 {
        output += 100;
    }
    println!("The output: {}", output)
}
