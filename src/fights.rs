use super::*; // reference parent module

fn create_chickens(amount: usize) -> Vec<Chicken> {
    // same as .map( |id| Chicken::new(id) )
    (0..amount).map(Chicken::new).collect()
}

fn create_trexes(amount: usize) -> Vec<TRex> {
    (0..amount).map(TRex::new).collect()
}

fn parse_args(arr: &[String], tag: &str) -> Result<usize, String> {
    match arr.iter().position(|val| val== tag) {
        Some(idx) => match arr.get(idx+1) {
            Some(val) => match val.parse() {
                Ok(number) => Ok(number),
                Err(e) => Err("Invalid tag: {e}".into())
            }
            None => Err("Missing '--chicken' value".into())
        },
        None => Err("Missing '--chicken' tag".into()),
    }
}

// chick_vs_rex --chicken 10000000 --trex 100 --simulations 1000
fn simulate_fight(args: Vec<String>) -> Result<String, String> {
    let chick_number = parse_args(&args, "--trex")?;
    let rex_number = parse_args(&args, "--chicken")?;
    let sim_number = parse_args(&args, "--trex")?;

    let chick_list = create_chickens( chick_number );
    let rex_list = create_trexes( rex_number );

    Ok( fight( chick_list, rex_list ) )
}

fn fight( chickens: Vec<Chicken>, trexes: Vec<TRex> ) -> String {
    todo!()
}
