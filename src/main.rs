extern crate word_fuzzer;

use word_fuzzer::fuzz;
use std::env;

fn main() {
    for argument in env::args().skip(1) {
        print!("{} ", fuzz(&argument));
    }
    println!("");
}
