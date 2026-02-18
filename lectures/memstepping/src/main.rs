fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
    let mut i = low;
    while i <= high {
        *total += i;
        i += step;
    }
}

fn main() {
    
}