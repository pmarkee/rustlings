// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase_with_borrow(mut data: &String) {
    // data = &data.to_uppercase();
    // This is error[E0716]: temporary value dropped while borrowed.
    // I don't understand why, `rustc --explain E0716` says the temporary
    // value's scope will be extended to the end of the enclosing block
    // if it is immediately assigned into a variable, which here it is.
    // If we use a `let` it works though. Or if we own `data` like the exercise calls for.
    // I wanted to leave this variation here though because I find it interesting.
    let new_data = &data.to_uppercase();

    println!("{}", new_data);
}

fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
    println!("{}", data);
}
