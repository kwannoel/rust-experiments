fn main() {
    let args: Vec<String> = env::args().collect();
    let a1 = args[0].parse::<usize>().unwrap();
    let len1 = args[1].parse::<usize>().unwrap();
    let a2 = args[2].parse::<usize>().unwrap();
    let len2 = args[3].parse::<usize>().unwrap();
    let vec1 = vec![a1; len1];
    let vec2 = vec![a2; len2];
    if vec1 == vec2 {
        println!("equivalent");
    } else {
        println!("diff");
    }
}