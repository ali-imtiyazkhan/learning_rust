fn main() {
    println!("1) String vs &str:");
    let s_owned = String::from("hello");
    let s_borrowed: &str = "world";
    println!("  owned: {}, borrowed: {}", s_owned, s_borrowed);

    println!("\n2) push_str and push:");
    let mut s = s_owned.clone();
    s.push_str(", ");
    s.push('R');
    s.push_str("ust");
    println!("  after push_str/push: {}", s);

    println!("\n3) Concatenation with + (moves left operand):");
    let s1 = String::from("Good");
    let s2 = String::from("bye");
    let s3 = s1 + " " + &s2; // s1 is moved, s2 is borrowed
    println!("  s3: {}", s3);

    println!("\n4) format! (no moves):");
    let formatted = format!("{}-{}", s2, "2026");
    println!("  format!: {}", formatted);

    println!("\n5) Iterating chars and bytes (unicode-aware):");
    let sample = "नमस्ते";
    print!("  chars: ");
    for c in sample.chars() { print!("{} ", c); }
    println!();
    print!("  bytes: ");
    for b in sample.bytes() { print!("{} ", b); }
    println!();

    println!("\n6) Slicing &str safely via chars().take():");
    let hello = "Hello, world";
    let first5: String = hello.chars().take(5).collect();
    println!("  first 5 chars: {}", first5);

    println!("\n7) Common helpers:");
    let trimmed = "  spaced  ".trim();
    println!("  trimmed: {:?}", trimmed);
    println!("  upper: {}", "rust".to_uppercase());
    println!("  replaced: {}", "a-b-c".replace("-", "_"));

    println!("\n8) Converting between String and bytes:");
    let bytes = "rust".as_bytes();
    println!("  bytes: {:?}", bytes);
}
