#![feature(negative_impls)]

struct Custom {}

trait NotOrd {}

impl !Ord for Custom {}

struct AllowOrd<T>(T);

impl PartialEq for AllowOrd<Custom> {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Eq for AllowOrd<Custom> {
}

impl PartialOrd for AllowOrd<Custom> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

impl Ord for AllowOrd<Custom> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

fn main() {

}