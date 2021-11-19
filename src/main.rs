use owo_colors::OwoColorize;
use std::io::Write;

mod gen;

use gen::*;

fn main() {
    // Generate steps and enemies' positions.
    let player_steps = gen_player_steps();
    let enemies_steps = gen_enemies_steps();
    let (west_enemy, east_enemy) = gen_enemies_positions(enemies_steps[0], true);
    let (north_enemy, south_enemy) = gen_enemies_positions(enemies_steps[1], false);

    // Print them.
    print_field(
        player_steps,
        west_enemy,
        east_enemy,
        north_enemy,
        south_enemy,
    );

    let (first_player_column, first_player_row) = ask_player_step(true);
    println!("");

    let west_enemy_pos: u8 = west_enemy.direction.into();
    let west_row = if west_enemy_pos == 2 {
        west_enemy_pos + west_enemy.steps.0
    } else {
        west_enemy_pos - west_enemy.steps.0
    };
    let east_enemy_pos: u8 = east_enemy.direction.into();
    let east_row = if east_enemy_pos == 2 {
        east_enemy_pos + east_enemy.steps.0
    } else {
        east_enemy_pos - east_enemy.steps.0
    };

    if west_row == first_player_row || east_row == first_player_row {
        println!("You were hit by the attack X(");
        println!("{}", "Game Over!".red());
        std::process::exit(0);
    } else {
        println!("{}", "You dodged the attack, great!".green());
    }

    let (second_player_column, second_player_row) = ask_player_step(false);
    println!("");

    let moved_column = std::cmp::max(second_player_column.saturating_sub(first_player_column), first_player_column.saturating_sub(second_player_column));
    let moved_row = std::cmp::max(second_player_row.saturating_sub(first_player_row), first_player_row.saturating_sub(second_player_row));
    if player_steps.0.0 != moved_column + moved_row {
        println!("{}", "Failed to pass the first debuff X(".red());
        println!("{}", "Game Over!".red());
        std::process::exit(0);
    }

    let north_enemy_pos: u8 = north_enemy.direction.into();
    let north_column = if north_enemy_pos == 2 {
        north_enemy_pos + north_enemy.steps.0
    } else {
        north_enemy_pos - north_enemy.steps.0
    };
    let south_enemy_pos: u8 = south_enemy.direction.into();
    let south_column = if south_enemy_pos == 2 {
        south_enemy_pos + south_enemy.steps.0
    } else {
        south_enemy_pos - south_enemy.steps.0
    };
    if north_column == second_player_column || south_column == second_player_column {
        println!("{}", "You were hit by the attack X(".red());
        println!("{}", "Game Over!".red());
        std::process::exit(0);
    } else {
        println!("{}", "You dodged the second attack and passed the first debuff...".green());
    }

    println!("\nFinally, all you need is go to the goal, run, run, run!\n\n");

    let moved_column = std::cmp::max(second_player_column.saturating_sub(3), 3_u8.saturating_sub(second_player_column));
    let moved_row = std::cmp::max(second_player_row.saturating_sub(1), 1_u8.saturating_sub(second_player_row));
    if player_steps.1.0 - player_steps.0.0 != moved_column + moved_row {
        println!("{}", "Failed to pass the second debuff X(".red());
        println!("{}", "Game Over!".red());
        std::process::exit(0);
    }

    println!("{}", "Hooray! Passed the second debuff, you survived!!!".green());
}

/// Pretty-print the field.
fn print_field(
    player_steps: (Steps, Steps),
    west_enemy: Position,
    east_enemy: Position,
    north_enemy: Position,
    south_enemy: Position,
) {
    println!(
        "The queen gave you elects...\nFirst: {}\nSecond: {}\n",
        player_steps.0.0, player_steps.1.0 - player_steps.0.0
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

fn ask_player_step(is_first: bool) -> (u8, u8) {
    if is_first {
        println!("\nEnter your first position you want to go.");
        println!("syntax: column row");
        println!("example: 4 2\n");
    } else {
        println!("\nEnter your second position you want to go.\n");
    }
    print!("> ");

    let mut line = String::new();
    let mut steps: Vec<u8> = Vec::new();

    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("failed to read your input, exiting...");

    for (n, c) in line.trim().split_whitespace().enumerate() {
        if n > 1 {
            println!("you specified more than two numbers, ignoring...");
            break;
        }
        let input: u8 = c
            .parse()
            .expect("failed to parse your input, make sure your input's range is 1 to 5");
        if !(1..=5).contains(&input) {
            eprintln!("the input's range has to be 1 to 5, exiting...");
            std::process::exit(-1);
        }
        steps.push(input);
    }

    assert!(steps.len() == 2, "you have to specify two numbers, exiting...");

    (steps[0], steps[1])
}
