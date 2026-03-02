/*
    - Create an array of 10 ints of choice
    - Implement the function is_even(n: i32) -> bool which returns true if a number is even, false otherwise
    - Use a for loop to iterate through the array for each number
    - Print whether it's even or odd using the is_even function
    - if the number is sivisible by 3, print Fizz.
    - if the number is divisible by 5, pring Buzz.
    - If it's divisible by both 3 AND 5, print FizzBuzz.
    - Use a while loop to find and print the sum of all the ints in the array. 
    - Use a loop to find and print the largest number in the array.

*/

fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let ints: [i32; 10] = [1, 3, 5, 7, 9, 2, 4, 6, 8, 15];
    for n in 0..ints.len() {
        println!("Is {} is divisible by 2? {}", ints[n], is_even(ints[n] as i32));
        if ints[n] % 3 == 0 && ints[n] % 5 == 0 {
            println!("FizzBuzz");
        } else if ints[n] % 3 == 0 {
            println!("Fizz");
        } else if ints[n] % 5 == 0 {
            println!("Buzz");
        }
    }
    let mut sum = 0;
    let mut n = 0;
    while n < ints.len() {
        sum += ints[n];
        n += 1;
    }
    println!("Sum of array: {}", sum);
    n = 0;
    let mut current = 0;
    while n < ints.len() {
        if ints[n] > current {
            current = ints[n];
        }
        n += 1;
    }
    println!("The largest integer in the array is: {}", current);
}
