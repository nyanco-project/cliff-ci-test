fn print(message: impl AsRef<str> + std::fmt::Display) {
   println!("{}", message);
}

fn main() {
    print("Test multiple PRs.");
}
