use std::fs;

fn main() {
    fs::File::open("./README.md").expect("opening-readme");
    fs::File::open("../avoum-phase-3/my-sudt/build/release/my-sudt").expect("opening my sudt");
    // fs::File::open("./my-sudt").expect("opening my sudt");
}
