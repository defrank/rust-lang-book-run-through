fn main() {
    println!("Scalar types!");
    let decimal: isize = 98_222;
    let hex: usize = 0xff;
    let octal: i16 = 0o77;
    let binary: u16 = 0b1111_0000;
    let byte: u8 = b'A';
    println!("{} {} {} {} {}", decimal, hex, octal, binary, byte);
    println!();
    println!("Integer overflow!");
    assert_eq!((i32::MAX - 2).checked_add(1), Some(i32::MAX - 1));
    assert_eq!((i32::MAX - 2).checked_add(3), None);
    println!();
    println!("Floating-point types!");
    let x = 2.0;  // f64
    let y: f32 = 3.0;  // f32
    println!("{} {}", x, y);
    println!();
    println!("Numeric operations!");
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4*30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("{} {} {} {} {}", sum, difference, product, quotient, remainder);
    println!();
    println!("The boolean type!");
    let t = true;
    let f: bool = false;
    println!("{} {}", t, f);
    println!();
    println!("The character type!");
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{} {} {}", c, z, heart_eyed_cat);
}
