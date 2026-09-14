use super::*; // reference parent module

fn create_chickens(amount: usize) -> Vec<Chicken> {
    // same as .map( |id| Chicken::new(id) )
    (0..amount).map(|_| Chicken::new()).collect()
}

fn create_trexes(amount: usize) -> Vec<TRex> {
    (0..amount).map(|_| TRex::new()).collect()
}

// can use a generic animal typs here also
fn create_animal<A: Animal>(amount: usize) -> Vec<A> {
    (0..amount).map(|_| A::new()).collect::<Vec<A>>()
}

// example for doc comments with tests for a project
/// Take CLI arguements and parses them.
///
/// Checks if we have chicken and tres values inputed
/// ```rust
/// use chick_vs_rex::fights::*;
/// let args = parse_args( &["chicken".into(),
///     "1000".into(), "trex".into(), "10".into()], "trex" );
/// assert!( args.is_ok() );
/// ```
/// Clap is another handly Lib that help with env variavble parcing.
pub fn parse_args(arr: &[String], tag: &str) -> Result<usize, String> {
    match arr.iter().position(|val| val == tag) {
        Some(idx) if arr[idx] == "--print" => match arr.get(idx + 1) { 
            Some(val) => match val.as_str() {
                "true" => Ok(1),
                _ => Err("Not correct value: setting to false".into()),
            },
            None => Err("Missing print: Defaulted to false".into())
        },
        Some(idx) => match arr.get(idx + 1) {
            Some(val) => match val.parse() {
                Ok(0) => Err("Invalid. Cant be 0".into()),
                Ok(num) => Ok(num),
                Err(e) => Err("Invalid tag: {e}".into()),
            },
            None => Err(format!("Missing '{}' value", tag)),
        },
        None => Err(format!("Missing '{}' tag", tag)),
    }
}

// gives result type for our lib
pub fn get_fight_results(args: Vec<String>) -> Result<String, String> {
    let chick_number = parse_args(&args, "--chicken")?;
    let rex_number = parse_args(&args, "--trex")?;
    let sim_number = parse_args(&args, "--sim").unwrap_or(1);
    let can_print = parse_args(&args, "--print").is_ok();

    let chick_list = create_chickens(chick_number);
    let rex_list = create_trexes(rex_number);

    // can use a generic fn and define the animal type
    // let animal_list = create_animal::<TRex>(1000000);

    Ok(sim_fight(chick_list, rex_list, can_print))
}

fn vol_to_sur(vol: u32) -> u32 {
    let radius: f32 = ((3.0 * (vol as f32)) / (4.0 * std::f32::consts::PI)).powf(1.0 / 3.0);
    ((4.0 * std::f32::consts::PI) * radius.powi(2)) as u32
}

fn group_fight<'a, A1, A2>(pair: (&'a mut [A1], &'a mut [A2]), timer: u32)
where
    A1: Animal + std::fmt::Debug + Sized,
    A2: Animal + std::fmt::Debug + Sized,
{
    let mut peek_a1 = pair.0.iter().enumerate().peekable();
    let mut peek_a2 = pair.1.iter().enumerate().peekable();

    let pairs: Vec<_> = std::iter::from_fn(move || {
        let ani1 = peek_a1.next()?;
        let ani2 = peek_a2.next()?;

        let mut idx_end1 = ani1.0;
        let mut idx_end2 = ani2.0;

        let mut total_a1 = ani1.1.size();
        let mut total_a2 = ani2.1.size();

        let val = if total_a1 < total_a2 {
            while total_a1 < total_a2 {
                let mut next = match peek_a1.peek() {
                    Some(val) => val,
                    _ => break,
                };
                let new_total: u32 = (((total_a1 + next.1.size()) as f32) * 1.03f32) as u32;
                if new_total < total_a2 {
                    let next = peek_a1.next()?;
                    total_a1 = new_total;
                    idx_end1 = next.0;
                } else {
                    break;
                }
            }
            // (&mut a1[ani1_start..=ani1.0], &mut a2[ani2_start..=ani2.0])
            // println!("1------: ani1 {}, idx_end1 {} ani2 {}, idx_end2 {}", ani1.0, idx_end1, ani2.0, idx_end2);
            ((ani1.0..=idx_end1), (ani2.0..=idx_end2))
        } else if total_a2 < total_a1 {
            while total_a2 < total_a1 {
                let mut next = match peek_a2.peek() {
                    Some(val) => val,
                    _ => break,
                };
                let new_total = total_a2 + next.1.size();
                if new_total < total_a1 {
                    let next = peek_a2.next()?;
                    total_a2 += next.1.size();
                    idx_end2 = next.0;
                } else {
                    break;
                }
            }
            // println!("2: ani1 {}, idx_end1 {} ani2 {}, idx_end2 {}", ani1.0, idx_end1, ani2.0, idx_end2);
            ((ani1.0..=idx_end1), (ani2.0..=idx_end2))
        } else {
            // println!("3: ani1 {}, idx_end1 {} ani2 {}, idx_end2 {}", ani1.0, idx_end1, ani2.0, idx_end2);
            ((ani1.0..=idx_end1), (ani2.0..=idx_end2))
        };

        Some(val)
    })
    .collect();

    pairs.iter().for_each(|ani_pairs| {
        let (animal1, animal2) = (
            &mut pair.0[ani_pairs.0.clone()],
            &mut pair.1[ani_pairs.1.clone()],
        );
        // println!("animal len {}, {}", animal1.len(), animal2.len());
        animal1.iter_mut().for_each(|type1| {
            animal2.iter_mut().for_each(|type2| {
                if timer.is_multiple_of(type1.speed()) {
                    type2.damage(type1.attack());
                    // println!("type2 health {}", type2.health());
                }
                if timer.is_multiple_of(type2.speed()) {
                    type1.damage(type2.attack());
                    // println!("type1 health {}", type1.health());
                }
            });
        })
    });
}

fn sim_fight<A1, A2>(mut animal1: Vec<A1>, mut animal2: Vec<A2>, print: bool) -> String
where
    A1: Animal + std::fmt::Debug + Sized,
    A2: Animal + std::fmt::Debug + Sized,
{
    let mut current_time = 0;
    let mut ani1group = AnimalGroup::new(&mut animal1);
    let mut ani2group = AnimalGroup::new(&mut animal2);

    while !ani1group.empty() && !ani2group.empty() {
        current_time += 1;

        // let ani_tuple: (&mut [A1], &mut [A2]) = (&mut animal1, &mut animal2);
        let ani_tuple: (&mut [A1], &mut [A2]) = (ani1group.list, ani2group.list);
        group_fight(ani_tuple, current_time);

        // animal1.retain(|animal1| animal1.health() > 0.0);
        ani1group.update();
        ani2group.update();
    }
    
    get_print(&animal1, &animal2, print, current_time )
}

fn get_print<A1, A2>(ani1: &[A1], ani2: &[A2], print: bool, round: u32 ) -> String
where
    A1: Animal + std::fmt::Debug + Sized,
    A2: Animal + std::fmt::Debug + Sized,
{
    let ani1name = A1::name();
    let ani2name = A2::name();

    if !print {
        println!("-----Winner----");
        if ani1.is_empty() {
            return format!("\"{}\" win the Fight!",ani2name)
        } else {
            return format!("\"{}\" win the Fight!",ani1name)
        }
    }
    
    let a1_ini = ani1.iter().map(|a| a.health()).sum::<f32>();
    let a2_ini = ani2.iter().map(|a| a.health()).sum::<f32>();

    let a1_count = ani1.len();
    let a2_count = ani2.len();

    println!("\nani1: {} --", ani1name);
    println!(
        "Health for all \"{}\". Start: {}  End: {}",
        ani1name,
        a1_ini,
        ani1.iter().map(|a| a.health()).sum::<f32>().abs()
    );
    println!("Number of \"{}\". Start: {}  End: {}\n", 
        ani1name,
        a1_count,
        ani1.len());

    println!("ani2: {} --", A2::name());
    println!(
        "Health for all \"{}\". Start: {}  End: {}",
        ani2name,
        a1_ini,
        ani2.iter().map(|a| a.health()).sum::<f32>().abs()
    );
    println!("Number of \"{}\". Start: {}  End: {}\n", 
        ani2name,
        a2_count,
        ani2.len());

    println!("-----Winner----");
    println!("Number of rounds fought {}", round);
    if ani1.is_empty() {
        format!("\"{}\" win the Fight!",ani2name)
    } else {
        format!("\"{}\" win the Fight!",ani1name)
    }
}

// how to use unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chicken_list() {
        let chicken_list = create_chickens(100);
        assert_eq!( chicken_list.len(), 100 );

        let size_avg = chicken_list.iter().map( |chicken| {
            chicken.size() 
        }).sum::<u32>();
        assert!( (100..=200).contains(&size_avg) );
    }
}

