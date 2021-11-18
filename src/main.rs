use std::fmt::format;

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
fn gen_player_steps() -> (Step, Step) {
    let mut rng = rand::thread_rng();

    // SAFETY: unwraps are safe as the slices aren't empty.
    let first_step: u8 = *[2, 3, 4].choose(&mut rng).unwrap();
    let sum: u8 = *[5, 6, 7].choose(&mut rng).unwrap();

    (Step(first_step), Step(sum))
}

/// Generate the number of steps that the enemies will move.
///
/// ## Note
/// - The range of steps is 1 to 3.
/// - (1, 1) or (3, 3) won't happen.
fn gen_enemies_steps() -> Vec<(Step, Step)> {
    let mut rng = rand::thread_rng();

    let mut a = Vec::new();
    while a.len() < 2 {
        // SAFETY: unwraps are safe as the slices aren't empty.
        let num_0: u8 = *[1, 2, 3].choose(&mut rng).unwrap();
        let num_1: u8 = *[1, 2, 3].choose(&mut rng).unwrap();
        if (num_0, num_1) != (1, 1) && (num_0, num_1) != (3, 3) {
            a.push((Step(num_0), Step(num_1)));
        }
    }

    a
}

/// Pretty-print the field.
fn print_field() {
    let player_steps = gen_player_steps();
    let enemies_steps = gen_enemies_steps();
    let (west_enemy, east_enemy) = gen_enemies_positions(enemies_steps[0], true);
    let (north_enemy, south_enemy) = gen_enemies_positions(enemies_steps[1], false);
    println!(
        "player steps: {:?}\nenemy steps: {:?}",
        player_steps, enemies_steps
    );

    for i in 0..=6 {
        match i {
            0 => {
                if 2 == north_enemy.direction.into() {
                    println!("      {} {}", north_enemy.steps, north_enemy.direction);
                } else {
                    println!("        {} {}", north_enemy.direction, north_enemy.steps);
                };
            }
            1 => {
                println!("    １２３４５");
                println!("   {}{}{}{}", i, "・・".red(), "・".green(), "・・".red());
            }
            2 if i == west_enemy.direction.into() => {
                println!(" {} {}{}", west_enemy.steps, i, "・・・・・".red());
            }
            2 => {
                println!("   {}{}{}", i, "・・・・・".red(), east_enemy.steps);
            }
            3 => println!(
                " {} {}{}{}",
                west_enemy.direction,
                i,
                "・・・・・".red(),
                east_enemy.direction
            ),
            4 if i == west_enemy.direction.into() => {
                println!(" {} {}{}", west_enemy.steps, i, "・・・・・".red());
            }
            4 => {
                println!("   {}{}{}", i, "・・・・・".red(), east_enemy.steps);
            }
            6 => {
                if 2 == south_enemy.direction.into() {
                    println!("      {} {}", south_enemy.steps, south_enemy.direction);
                } else {
                    println!("        {} {}", south_enemy.direction, south_enemy.steps);
                };
            }
            _ => println!("   {}{}", i, "・・・・・".red()),
        }
    }
    println!("\n({} = goal)", "・".green());
}

#[derive(Debug, Clone, Copy)]
struct Position {
    steps: Step,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Upward,
    Downward,
    Leftward,
    Rightward,
}

#[derive(Debug, Clone, Copy)]
struct Step(u8);

impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            1 => write!(f, "➀"),
            2 => write!(f, "②"),
            3 => write!(f, "➂"),
            _ => panic!("This shouldn't be happened!"),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Direction::Upward => write!(f, "↑"),
            Direction::Downward => write!(f, "↓"),
            Direction::Leftward => write!(f, "←"),
            Direction::Rightward => write!(f, "→"),
        }
    }
}

impl Into<i32> for Direction {
    fn into(self) -> i32 {
        match self {
            Direction::Upward => 4,
            Direction::Downward => 2,
            Direction::Leftward => 4,
            Direction::Rightward => 2,
        }
    }
}

fn gen_enemies_positions(steps: (Step, Step), east_and_west: bool) -> (Position, Position) {
    let mut rng = rand::thread_rng();

    // SAFETY: Unwrap here is safe as the slice isn't empty.
    let n = *[0, 1].choose(&mut rng).unwrap();

    let (direction, second_direction) = if n == 0 {
        if east_and_west {
            (Direction::Downward, Direction::Upward)
        } else {
            (Direction::Rightward, Direction::Leftward)
        }
    } else {
        if east_and_west {
            (Direction::Upward, Direction::Downward)
        } else {
            (Direction::Leftward, Direction::Rightward)
        }
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
