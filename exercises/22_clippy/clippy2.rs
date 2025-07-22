fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    // Clippy suggests using `if let` or `while let` to handle the `Option`.
    // This will help avoid unnecessary pattern matching.
    // Instead of using `match`, we can use `if let` to handle the `Some` case.
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
