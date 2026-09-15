use orx_parallel::*;
use orx_soa::soa2::Soa2;
use std::{hint::black_box, time::Instant};

const NUM_ITEMS: usize = 1 << 24;

fn work(value: usize) -> u64 {
    let mut result = value as u64;
    for _ in 0..128 {
        result = black_box(result.wrapping_mul(1_664_525).wrapping_add(1_013_904_223));
        result ^= result >> 13;
    }
    result
}

/// This example demonstrates how to use SOA collections as a destination for extending by a
/// parallel iterator.
///
/// Notice that the usage is identical to collecting into a `Vec` or other collections.
///
/// You may test it by:
///
/// ```sh
/// cargo run --release --example par_collect -- 1
/// cargo run --release --example par_collect -- 4
/// ```
fn main() {
    let mut args = std::env::args().skip(1);
    let num_threads = args
        .next()
        .expect("usage: cargo run --release --features std --example par_collect -- <num-threads>")
        .parse::<usize>()
        .expect("num-threads must be a positive integer");
    assert!(num_threads > 0, "num-threads must be a positive integer");

    let soa: Soa2<usize, u64> = (0..NUM_ITEMS)
        .par()
        .map(|value| (value, black_box(work(value))))
        .num_threads(num_threads)
        .collect();

    let checksum: u64 = soa.as_slice2().iter().copied().sum();
    println!(
        "collected {} pairs with {num_threads} thread(s), checksum={checksum}",
        soa.len()
    );

    let begin = Instant::now();

    for _ in 0..10 {
        let soa: Soa2<usize, u64> = (0..NUM_ITEMS)
            .par()
            .map(|value| (value, black_box(work(value))))
            .num_threads(num_threads)
            .collect();

        let checksum: u64 = soa.as_slice2().iter().copied().sum();
        println!(
            "collected {} pairs with {num_threads} thread(s), checksum={checksum}",
            soa.len()
        );
    }

    let elapsed = begin.elapsed();
    println!("{elapsed:?}");
}
