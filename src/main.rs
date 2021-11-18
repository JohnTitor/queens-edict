use owo_colors::OwoColorize;

mod gen;

use gen::*;

fn main() {
    print_field();
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
