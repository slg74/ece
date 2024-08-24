use std::f64;

struct SpecialFloats {
    positive_infinity: f64,
    negative_infinity: f64,
    smallest_positive: f64,
    underflow_to_zero: f64,
}

fn main() {
    let special_floats = SpecialFloats {
        positive_infinity: f64::INFINITY,
        negative_infinity: f64::NEG_INFINITY,
        smallest_positive: f64::MIN_POSITIVE,
        underflow_to_zero: 1e-325,
    };

    println!("Special IEEE 754 examples:"); 

    println!(" positive infinity: ");
    print_ieee754(special_floats.positive_infinity);

    println!("\n negative infinity: ");
    print_ieee754(special_floats.negative_infinity);

    println!("\n smallest positive number: ");
    print_ieee754(special_floats.smallest_positive);

    println!("\n underflow to zero: ");
    print_ieee754(special_floats.underflow_to_zero);
}

fn print_ieee754(num: f64) {
    let binary = to_ieee754_binary(num);
    println!("Value: {}", num);
    println!("IEEE 754: {}", binary);
}

fn to_ieee754_binary(num: f64) -> String {
    let bits = num.to_bits();
    let mut binary = format!("{:064b}", bits);

    // Insert spaces for readability
    binary.insert(1, ' '); // After sign bit
    binary.insert(13, ' '); // After exponent
    binary
}