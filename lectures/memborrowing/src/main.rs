// Referencing and De-Referencing
    /*
fn referencing() {
    let x: i32 = 5;
    let y = &x; //reference
    let &z = y;
    println!("z = {}, y = {}, x = {}", z, y, x);
}
    */

// Borrowing Issue
fn borrowref() {
    let mut x = 5;
    let y = &x;
    println!("y = {}, x = {}", y, x);

    x += 1;
}

fn main() {
    // referencing();
    borrowref();
}