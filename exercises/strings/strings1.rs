// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    // Using `to_string` to convert the `&str` to `String`
    "blue".to_string()

    // Using `String::from` to convert the `&str` to `String`
    // String::from("blue")
}
