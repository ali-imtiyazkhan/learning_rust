fn main() {
    // --- Integers ---
    // Signed integers: i8, i16, i32, i64, i128, isize
    // Unsigned integers: u8, u16, u32, u64, u128, usize

    let a: i32 = -123;
    let b: u32 = 123;
    
    // isize and usize depend on the architecture (32-bit or 64-bit)
    let arch_size: usize = 10;

    // --- Floats ---
    // f32 and f64 (default is f64)
    let f: f64 = 3.14;
    let g: f32 = 2.71;

    // --- Number Literals ---
    let decimal = 98_222;       // Decimal
    let hex = 0xff;             // Hexadecimal
    let octal = 0o77;           // Octal
    let binary = 0b1111_0000;   // Binary
    let byte = b'A';            // Byte (u8 only)

    println!("Decimal: {}", decimal);
    println!("Hex: {}", hex);
    println!("Octal: {}", octal);
    println!("Binary: {}", binary);
    println!("Byte: {}", byte);

    // --- Arithmetic Operations ---
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("Sum: {}, Diff: {}, Prod: {}, Quo: {}, Rem: {}", sum, difference, product, quotient, remainder);
}
