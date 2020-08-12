// functions4.rs
// Make me compile! Execute `rustlings hint functions4` for hints :)

// This store is having a sale where if the price is an even number, you get
// 10 Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.

type int = i64;

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: int) -> int {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: int) -> bool {
    num % 2 == 0
}
