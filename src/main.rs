#![allow(clippy::let_unit_value)]
use chick_vs_rex::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // let result = match prepare_fight( args ) {
    //     Ok(res) => res,
    //     Err(e) => {
    //         eprintln!("{e}");
    //         std::process::exit(1);
    //     },
    // };

    // println!( "{result}" );

    let x = (0..1_000_000).collect::<Vec<u128>>();
    let val = std::time::Instant::now();
    sum_of_sq( &x );
    let val = val.elapsed();
    println!("Sum of squares for 1,000,000:");
    println!("time: {:?}", val);

    let val = std::time::Instant::now();
    sum_of_sq_rayon( &x );
    let val = val.elapsed();
    println!("time with rayon: {:?}", val);
}




fn sum_of_sq( input: &[u128] ) -> u128 {
    input
        .iter()
        .map( |val| val*val )
        .sum()
}


use rayon::prelude::*;
// Parallel processing for Iterators
fn sum_of_sq_rayon( input: &[u128] ) -> u128 {
    input
        .par_iter()
        .map( |val| val*val )
        .sum()
}
