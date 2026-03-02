// Constants for freeze point
const FREEZE_POINT_F: f64 = 32.0;

// F -> C function
fn fahrenheit_to_celsius(f: f64) -> f64 {
    return (f - FREEZE_POINT_F) * 5.0/9.0;
}

// C -> F function
fn celsius_to_fahrenheit(c: f64) -> f64 {
    return (c * 9.0/5.0) + FREEZE_POINT_F;
}

fn main() {
    let temp_in_fahrenheit: u32 = 85;
    println!("The temperature of {} F is: {} C", temp_in_fahrenheit, fahrenheit_to_celsius(temp_in_fahrenheit as f64) as u32);

    for n in 32..37 {
        println!("The temperature of {} F is {} C", n, fahrenheit_to_celsius(n as f64) as u32);
    }
}
