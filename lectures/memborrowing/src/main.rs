// Referencing and De-Referencing
    /*
fn referencing() {
    let x: i32 = 5;
    let y = &x; //reference
    let &z = y;
    println!("z = {}, y = {}, x = {}", z, y, x);
}
    

// Borrowing Issue
fn borrowref() {
    let mut x = 5;
    let y = &x;
    println!("y = {}, x = {}", y, x);

    x += 1;
}
    */

fn concatstrings(s1: &String, s2: &String) -> String {
    let mut sc = s1.clone() += s2.clone();
    return sc;
}

fn main() {
    // referencing();
    // borrowref();
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concatstrings(&s1, &s2);
    println!("{}", result);
}