use rand::seq::SliceRandom;

fn main() {
    println!("player steps: {:?}\nenemy steps: {:?}", gen_player_steps(), gen_enemies_steps());
    print_field();
}

/// Generate the number of steps that the player should move.
///
/// ## Note
/// The implementation rule is:
/// - The range of the first step is 2 to 4.
/// - The sum of steps is 5 to 7.
/// - The second turn's steps can be calculated by `sum - first_step`.
fn gen_player_steps() -> (u8, u8) {
    let mut rng = rand::thread_rng();

    // SAFETY: unwraps are safe as the slices aren't empty.
    let first_step: u8 = *[2, 3, 4].choose(&mut rng).unwrap();
    let sum: u8 = *[5, 6, 7].choose(&mut rng).unwrap();

    (first_step, sum)
}

/// Generate the number of steps that the enemies will move.
///
/// ## Note
/// - The range of steps is 1 to 3.
/// - (1, 1) or (3, 3) won't happen.
fn gen_enemies_steps() -> Vec<(u8, u8)> {
    let mut rng = rand::thread_rng();

    let mut a = Vec::new();
    while a.len() < 2 {
        // SAFETY: unwraps are safe as the slices aren't empty.
        let num_0: u8 = *[1, 2, 3].choose(&mut rng).unwrap();
        let num_1: u8 = *[1, 2, 3].choose(&mut rng).unwrap();
        if (num_0, num_1) != (1, 1) && (num_0, num_1) != (3, 3) {
            a.push((num_0, num_1));
        }
    }

    a
}

/// Pretty-print the field.
fn print_field() {
    use owo_colors::OwoColorize;
    for i in 0..5 {
        if i == 0 {
            println!("\n{}{}{}", "・・".red(), "・".green(), "・・".red());
        } else {
            println!("{}", "・・・・・".red());
        }
    }
    println!("\n* {} = goal", "・".green());
}
