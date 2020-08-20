// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

mod macros {
    #[macro_export]
    macro_rules! my_macro {
        ($val:expr) => {
            println!("Check out my macro! {}", $val);
        };
    }
}

fn main() {
    my_macro!(777 + 8888);
}
