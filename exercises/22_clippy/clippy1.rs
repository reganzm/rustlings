// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises, the code will fail to compile when there are Clippy
// warnings. Check Clippy's suggestions from the output to solve the exercise.

fn main() {
    // TODO: Fix the Clippy lint in this line.
    // Clippy suggests using `f32::consts::PI` instead of a hardcoded value.
    // You can also use `std::f32::consts::PI` or `
    // `std::f32::consts::PI` to access the constant.
    // The radius is also hardcoded, so you can use a variable instead.
    // Clippy will suggest using `f32::powi` instead of `f32::pow`.
    // The area of a circle is calculated using the formula `π * r^2`.
    let pi = std::f32::consts::PI;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}
