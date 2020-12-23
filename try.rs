fn main() {
    println!("{:?}", prog());
    println!("{:?}", prog_nested_fors());
    println!("{:?}", prog_ctrl());
}

fn prog() -> Option<()> {
    err()?;
    return Some(());
}

fn err() -> Option<()> {
    None
}

fn prog_nested_fors() -> Option<()> {
    for _ in 1..2 {
        for _ in 1..2 {
            err()?;
        }
    }
    return Some(());
}

fn prog_ctrl() -> Option<()> {
    return Some(());
}
