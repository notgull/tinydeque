// MIT/Apache2 License

use clap::{App, Arg};
use std::collections::VecDeque;
use tinydeque::{ArrayDeque, TinyDeque};

fn main() {
    let app = App::new("tinydeque Benchmarks")
        .version("0.1.0")
        .author("not_a_seagull <jtnunley01@gmail.com>")
        .about("Benchmarking for tinydeque")
        .arg(
            Arg::with_name("elements")
                .short("e")
                .long("elements")
                .default_value("20")
                .help("Numbers of elements to process")
                .takes_value(true),
        )
        .get_matches();

    // benchmark pushing elements into the back
    let mut array_deque: ArrayDeque<[char; 40]> = ArrayDeque::new();
    let mut tiny_deque: TinyDeque<[char; 40]> = TinyDeque::new();
    let mut vec_deque: VecDeque<char> = VecDeque::new();
}
