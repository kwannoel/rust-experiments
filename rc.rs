// Rc stands for reference counted pointer
// The value can be cleaned up w/o references becoming invalid

// Usecase:
// Alloc to heap for READ op, can't determine which ref will finish using data last.
// If we did know, we could move ownership to that last ref
// It is only used for single-threaded scenarios
fn main() {}
