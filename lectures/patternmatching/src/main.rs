// Pattern Matching in Rust
// an example of pattern matching using match (like switch cases)
// and some looping

fn get_rgb(c:char) -> (u8,u8,u8) {
    match c {
        'R' => (255,0,0),
        'G' => (0,255,0),
        'B' => (0,0,255),
        _ => (0,0,0),
    }
}

fn main() {
    let letters:[char;3] = ['R','G','B'];

    for idx in 0..letters.len() {
        let (r,g,b) = get_rgb(letters[idx]);
        println!("RED intensity {}, GREEN intensity {}, BLUE intensity {}", r,g,b);
    }
}
