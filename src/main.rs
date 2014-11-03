#![feature(slicing_syntax)]

extern crate getopts;

use getopts::{optopt, optflag, getopts, usage};
use std::iter::range_step_inclusive;
use std::f64;
use std::os;


fn compute_n_steps(start: uint, end: uint) -> f64 {
    let mut pi = 0.0_f64;
    let mut step = if (start - 1) % 4 == 0 {
        1.0_f64
    } else {
        -1.0_f64
    };
    for d in range_step_inclusive(start, end, 2) {
        pi += step / (d as f64);
        step = -step;
    }
    return pi;
}

fn compute(steps: uint) -> f64 {
    let mut pi = 0.0_f64;
    let (sender, receiver): (Sender<f64>, Receiver<f64>) = channel();
    let cpus = os::num_cpus();
    let split_steps = steps / cpus;

    for cpu in range(0, cpus) {
        let child_sender = sender.clone();
        let start = split_steps * cpu + 1;
        let end = start + split_steps - 1;

        spawn(proc() {
            child_sender.send(compute_n_steps(start, end));
        });
    }
    for _ in range(0, cpus) {
        pi += receiver.recv();
    }
    return pi;
}

fn main() {
    let args: Vec<String> = os::args();
    let opts = [
        optopt("s", "steps", "Number of steps to compute pi [default=1000]", "1000"),
        optflag("o", "optimize", "Optimise the computation by using multicores"),
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

    let optimise = matches.opt_present("o");

    let mut pi = if optimise {
        compute(steps)
    } else {
        compute_n_steps(1, steps)
    };

    pi *= 4.0_f64;
    println!("{}", f64::to_string(pi));
}
