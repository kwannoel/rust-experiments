
fn f(i: usize) -> Option<usize> {
    if i == 0 {
        return None;
    }
    Some(i)
}

fn g(i: usize) -> Result<usize, ()> {
    if i == 0 {
        return Err(());
    }
    Ok(i)
}

fn main() {
    f(1);
    f(2);
    g(1);
}