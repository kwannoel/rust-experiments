struct Custom {}

// trait NotOrd {}
trait NotOrd {}

impl<T> NotOrd for T where T: PartialOrd {}

impl PartialEq for Custom {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Eq for Custom {
}

impl PartialOrd for Custom {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

impl Ord for Custom {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

fn main() {

}