// Store on heap rather than stack, don't know the size of data
//
// Usecase:
// - Transfer ownership of large amount of data
//   Ensures data won't be copied
// - Don't know size of data, e.g. Trait objects, recursive types
//
// Reference: https://doc.rust-lang.org/book/ch15-01-box.html

// Unknown size, cannot type check
// enum List {
//     Cons(i32, List),
//     Nil,
// }

use crate::BoxedList::*;
use crate::RefList::*;

enum BoxedList {
    BoxedCons(i32, Box<BoxedList>),
    BoxedNil,
}

enum RefList<'a> {
    RefCons(i32, &'a RefList<'a>),
    RefNil,
}

fn main() {
    // Unable to check from type, what is the size?
    // From the type above, Cons(i32, List) can recurse indefinitely
    // The following gives compile error
    let boxed_list = BoxedCons(1, Box::new(BoxedCons(2, Box::new(BoxedCons(3, Box::new(BoxedNil))))));
    let ref_list = RefCons(1, &RefCons(1, &RefCons(1, &RefCons(2, &RefNil))));
}
