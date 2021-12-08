// Interior Mutability Pattern
// Mutate data even when there are immutable references to that data
// Single ownership over the data it holds.
// Single threaded scenarios
//
// Usecase:
// When you have an immutable value, you can't borrow it mutably:
// fn main () {
//     let x = 5;
//     let y = &mut x; // invalid
// }

fn main() {
}
