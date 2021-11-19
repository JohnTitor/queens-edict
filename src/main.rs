use owo_colors::OwoColorize;
use std::io::Write;

mod gen;

use gen::*;

fn main() {
    // Generate steps and enemies' positions.
    let player_steps = gen_player_steps();
    let enemies_steps = gen_enemies_steps();
    let (west_enemy, east_enemy) = gen_enemies_poss(enemies_steps[0], true);
    let (north_enemy, south_enemy) = gen_enemies_poss(enemies_steps[1], false);

    // Print them.
    print_field(
        player_steps,
        west_enemy,
        east_enemy,
        north_enemy,
        south_enemy,
        None,
        true,
    );

    let (first_player_column, first_player_row) = ask_player_step(true);
    println!("");

    let west_enemy_pos: u8 = west_enemy.direction.into();
    let east_enemy_pos: u8 = east_enemy.direction.into();

    if !attack_check(
        west_enemy_pos,
        east_enemy_pos,
        west_enemy.steps.0,
        east_enemy.steps.0,
        first_player_row,
    ) {
        println!("You were hit by the attack X(");
        println!("{}", "Game Over!".red());
        std::process::exit(0);
    } else {
        println!("{}", "You dodged the attack, great!".green());
    }

    print_field(
        player_steps,
        west_enemy,
        east_enemy,
        north_enemy,
        south_enemy,
        Some((first_player_column, first_player_row)),
        false,
    );

    let (second_player_column, second_player_row) = ask_player_step(false);
    println!("");

    if !move_check(
        first_player_column,
        second_player_column,
        first_player_row,
        second_player_row,
        player_steps.0 .0,
    ) {
        println!("{}", "Failed to pass the first debuff X(".red());
        println!("{}", "Game Over!".red());
        std::process::exit(0);
    }

    let north_enemy_pos: u8 = north_enemy.direction.into();
    let south_enemy_pos: u8 = south_enemy.direction.into();

    if !attack_check(
        north_enemy_pos,
        south_enemy_pos,
        north_enemy.steps.0,
        south_enemy.steps.0,
        second_player_column,
    ) {
        println!("{}", "You were hit by the attack X(".red());
        println!("{}", "Game Over!".red());
        std::process::exit(0);
    } else {
        println!(
            "{}",
            "You dodged the second attack and passed the first debuff...".green()
        );
    }

    println!("\nFinally, all you need is go to the goal, run, run, run!\n\n");

    if !move_check(
        second_player_column,
        3,
        second_player_row,
        1,
        player_steps.1 .0,
    ) {
        println!("{}", "Failed to pass the second debuff X(".red());
        println!("{}", "Game Over!".red());
        std::process::exit(0);
    }

    println!(
        "{}",
        "Hooray! Passed the second debuff, you survived!!!".green()
    );
}

/// Pretty-print the field.
fn print_field(
    player_steps: (Steps, Steps),
    west_enemy: Pos,
    east_enemy: Pos,
    north_enemy: Pos,
    south_enemy: Pos,
    player_pos: Option<(u8, u8)>,
    is_first: bool,
) {
    if is_first {
        println!("The Queen: Lady of blades, I beseech you -- grant me the strength to overcome our oppressors! \
                    To reclaim what is rightfully ours!\n");
        println!(
            "The Queen gave you edict...\nFirst: {}\nSecond: {}\n",
            player_steps.0 .0, player_steps.1 .0
        );
    } else {
        println!("\nSecond move...");
        println!(
            "Your edict is...\nFirst: {}\nSecond: {}\n",
            player_steps.0 .0, player_steps.1 .0
        );
    }

    for i in 0..=6 {
        match i {
            0 => {
                if 2 == north_enemy.direction.into() {
                    println!("      {} {}", north_enemy.steps, north_enemy.direction);
                } else {
                    println!("        {} {}", north_enemy.direction, north_enemy.steps);
                };

                println!("    １２３４５");
            }
            1 => {
                if !is_first && player_pos.unwrap().1 == 1 {
                    print!("   {}", i);
                    for i in 1..=5 {
                        if i == player_pos.unwrap().0 {
                            print!("{}", "・".yellow());
                        } else if i == 3 {
                            print!("{}", "・".green());
                        } else {
                            print!("{}", "・".red());
                        }
                    }
                    println!("");
                } else {
                    println!("   {}{}{}{}", i, "・・".red(), "・".green(), "・・".red());
                }
            }
            2 if is_first => {
                if i == west_enemy.direction.into() {
                    println!(" {} {}{}", west_enemy.steps, i, "・・・・・".red())
                } else {
                    println!("   {}{}{}", i, "・・・・・".red(), east_enemy.steps);
                }
            }
            2 => {
                if player_pos.unwrap().1 == 2 {
                    print!("   {}", i);
                    for i in 1..=5 {
                        if i == player_pos.unwrap().0 {
                            print!("{}", "・".yellow());
                        } else {
                            print!("{}", "・".red());
                        }
                    }
                    println!("");
                } else {
                    println!("   {}{}", i, "・・・・・".red());
                }
            }
            3 if is_first => println!(
                " {} {}{}{}",
                west_enemy.direction,
                i,
                "・・・・・".red(),
                east_enemy.direction
            ),
            3 => {
                if player_pos.unwrap().1 == 3 {
                    print!("   {}", i);
                    for i in 1..=5 {
                        if i == player_pos.unwrap().0 {
                            print!("{}", "・".yellow());
                        } else {
                            print!("{}", "・".red());
                        }
                    }
                    println!("");
                } else {
                    println!("   {}{}", i, "・・・・・".red());
                }
            }
            4 if is_first => {
                if i == west_enemy.direction.into() {
                    println!(" {} {}{}", west_enemy.steps, i, "・・・・・".red());
                } else {
                    println!("   {}{}{}", i, "・・・・・".red(), east_enemy.steps);
                }
            }
            4 => {
                if player_pos.unwrap().1 == 4 {
                    print!("   {}", i);
                    for i in 1..=5 {
                        if i == player_pos.unwrap().0 {
                            print!("{}", "・".yellow());
                        } else {
                            print!("{}", "・".red());
                        }
                    }
                    println!("");
                } else {
                    println!("   {}{}", i, "・・・・・".red());
                }
            }
            5 => {
                if !is_first && player_pos.unwrap().1 == 5 {
                    print!("   {}", i);
                    for i in 1..=5 {
                        if i == player_pos.unwrap().0 {
                            print!("{}", "・".yellow());
                        } else {
                            print!("{}", "・".red());
                        }
                    }
                    println!("");
                } else {
                    println!("   {}{}", i, "・・・・・".red());
                }
            }
            6 => {
                if 2 == south_enemy.direction.into() {
                    println!("      {} {}", south_enemy.steps, south_enemy.direction);
                } else {
                    println!("        {} {}", south_enemy.direction, south_enemy.steps);
                };
            }
            _ => panic!("Shouldn't reach here!"),
        }
    }

    if is_first {
        println!("\n({} = goal)", "・".green());
    } else {
        println!("\n({} = goal, {} = you)", "・".green(), "・".yellow());
    }
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

    assert!(
        steps.len() == 2,
        "you have to specify two numbers, exiting..."
    );

    (steps[0], steps[1])
}

/// Checks if the player is hit by the enemies' attacks.
fn attack_check(
    enemy1_pos: u8,
    enemy2_pos: u8,
    enemy1_steps: u8,
    enemy2_steps: u8,
    player_pos: u8,
) -> bool {
    let enemy1_pos = if enemy1_pos == 2 {
        enemy1_pos + enemy1_steps
    } else {
        enemy1_pos - enemy1_steps
    };
    let enemy2_pos = if enemy2_pos == 2 {
        enemy2_pos + enemy2_steps
    } else {
        enemy2_pos - enemy2_steps
    };

    enemy1_pos != player_pos && enemy2_pos != player_pos
}

/// Checks if the player passes the debuffs.
fn move_check(
    before_column: u8,
    after_column: u8,
    before_row: u8,
    after_row: u8,
    debuff: u8,
) -> bool {
    let moved_column = std::cmp::max(
        before_column.saturating_sub(after_column),
        after_column.saturating_sub(before_column),
    );
    let moved_row = std::cmp::max(
        after_row.saturating_sub(before_row),
        before_row.saturating_sub(after_row),
    );

    debuff == moved_column + moved_row
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attack_check() {
        assert!(!attack_check(2, 4, 1, 3, 1));
        assert!(attack_check(2, 4, 1, 3, 2));
        assert!(!attack_check(2, 4, 1, 3, 3));
        assert!(attack_check(2, 4, 1, 3, 4));
        assert!(attack_check(2, 4, 1, 3, 5));
    }

    #[test]
    fn test_move_check() {
        assert!(move_check(2, 1, 5, 4, 2));
        assert!(move_check(1, 3, 4, 1, 5));

        assert!(move_check(1, 1, 5, 2, 3));
        assert!(move_check(1, 3, 2, 1, 3));
    }
}
