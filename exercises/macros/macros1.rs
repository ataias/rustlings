// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

macro_rules! my_macro {
    ($x:expr) => {
        println!("Check out my macro! {}", $x);
    };
}

fn main() {
    my_macro!(333);
}
