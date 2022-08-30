extern crate num_cpus;
use prime_data::PrimeData;
use std::{
    env,
    ops::RangeInclusive,
    thread::{self, JoinHandle},
    time::{Instant, Duration},
};

struct ThreadOutput {
    sum: u64,
    duration: Duration,
    range: RangeInclusive<u64>,
}

fn main() {
    let n: u64 = env::args().last().unwrap().parse().unwrap();

    println!("Calculating prime sum up to {n}");
    let start_time = Instant::now();

    // Calculate prime range per cpu
    let cpus = num_cpus::get() as u64 * 2;
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut offset = 0;
    for cpu in 0..cpus {
        let count = n / cpus;
        if cpu == cpus {
            ranges.push(offset..=n);
        } else {
            ranges.push(offset..=(offset + count));
        }
        offset += count + 1;
    }

    // Calculate primes
    let mut handles: Vec<JoinHandle<ThreadOutput>> = Vec::with_capacity(cpus as usize);
    for cpu in 0..cpus {
        let range = ranges[cpu as usize].clone();

        handles.push(thread::spawn(|| {
            let start_time = Instant::now();

            let prime_data = PrimeData::generate(range.clone());

            let mut sum = 0;
            for prime in prime_data.iter_all() {
                sum += prime;
            }

            ThreadOutput {
                sum: sum,
                duration: Instant::now().duration_since(start_time),
                range: range,
            }
        }));
    }

    // Sum primes
    let mut sum = 0;
    for handle in handles.into_iter() {
        let thread_output = handle.join().unwrap();

        sum += thread_output.sum;
        println!("{:?} - {:?}", thread_output.range, thread_output.duration);
    }

    println!("Duration: {:?}", Instant::now().duration_since(start_time));
    println!("Prime sum: {:?}", sum);
}
