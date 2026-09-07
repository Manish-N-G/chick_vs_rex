#![allow(clippy::let_unit_value)]
use chick_vs_rex::*;
use std::env;

fn main() {
    // entry point
    // let chick1 = Chicken::new(1);
    // let chick2 = Chicken::new(2);
    //
    // let rex1 = TRex::new(1);
    // println!("{:?}", chick1);
    // println!("{:?}", chick2);
    // println!("{:?}", rex1);

    let args: Vec<String> = env::args().skip(1).collect();

    let result = match prepare_fight( args ) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        },
    };

    println!( "{result}" );
}

use std::fs::*;
fn other() {



}
