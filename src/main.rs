#![allow(clippy::let_unit_value)]
use chick_vs_rex::*;
use std::env;

fn main() {
    //entry point
    let args: Vec<String> = env::args().skip(1).collect();

    let result = match get_fight_results( args ) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        },
    };

    println!( "{result}" );
}

