use owo_colors::OwoColorize;
use rand::{distributions::Standard, prelude::Distribution, seq::SliceRandom, Rng};

fn main() {
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
    let player_steps = gen_player_steps();
    let enemies_steps = gen_enemies_steps();
    let (west_enemy, east_enemy) = gen_enemies_positions(enemies_steps[0]);
    println!(
        "player steps: {:?}\nenemy steps: {:?}",
        player_steps, enemies_steps
    );

    for i in 1..=5 {
        match i {
            1 => {
                println!("\n     １２３４５");
                println!("    {}{}{}{}", i, "・・".red(), "・".green(), "・・".red());
            }
            2 if i == west_enemy.direction.into() => {
                println!("{} @ {}{}", west_enemy.steps, i, "・・・・・".red());
            }
            2 => {
                println!("    {}{}@ {}", i, "・・・・・".red(), east_enemy.steps);
            }
            3 => println!(
                "  {} {}{}{}",
                west_enemy.direction,
                i,
                "・・・・・".red(),
                east_enemy.direction
            ),
            4 if i == west_enemy.direction.into() => {
                println!("{} @ {}{}", west_enemy.steps, i, "・・・・・".red());
            }
            4 => {
                println!("    {}{}@ {}", i, "・・・・・".red(), east_enemy.steps);
            }
            _ => println!("    {}{}", i, "・・・・・".red()),
        }
    }
    println!("\n({} = goal)", "・".green());
}

#[derive(Debug, Clone, Copy)]
struct Position {
    steps: u8,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Upward,
    Downward,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Direction::Upward => write!(f, "↑"),
            Direction::Downward => write!(f, "↓"),
        }
    }
}

impl Into<i32> for Direction {
    fn into(self) -> i32 {
        match self {
            Direction::Upward => 4,
            Direction::Downward => 2,
        }
    }
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=1) {
            0 => Direction::Upward,
            _ => Direction::Downward,
        }
    }
}

fn gen_enemies_positions(steps: (u8, u8)) -> (Position, Position) {
    let direction: Direction = rand::random();
    let second_direction = if direction == Direction::Upward {
        Direction::Downward
    } else {
        Direction::Upward
    };

    (
        Position {
            steps: steps.0,
            direction,
        },
        Position {
            steps: steps.1,
            direction: second_direction,
        },
    )
}
