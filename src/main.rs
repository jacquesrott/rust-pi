#![feature(slicing_syntax)]

extern crate getopts;

use getopts::{optopt, optflag, getopts, usage};
use std::iter::range_step_inclusive;
use std::f64;
use std::os;


fn main() {
    let args: Vec<String> = os::args();
    let opts = [
        optopt("s", "steps", "Number of steps to compute pi [default=1000]", "1000"),
        optflag("h", "help", "print this help message and exit"),
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(e) => fail!(e.to_string()),
    };

    let steps = matches
        .opt_str("s")
        .map(|d| from_str::<uint>(d[])
             .expect("Invalid type, please specify an integer"))
        .unwrap_or(1000);

    if matches.opt_present("h") {
        println!("{}", usage("A Pi decimal computer.", &opts));
        return;
    }

    let mut pi = 0.0_f64;
    let mut step = 1.0_f64;
    for d in range_step_inclusive(1, n, 2) {
        pi += step / (d as f64);
        step = -step;
    }
    pi *= 4.0_f64;
    println!("{}", f64::to_string(pi));
}
