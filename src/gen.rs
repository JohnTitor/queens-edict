use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Position {
    pub steps: Steps,
    pub direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Direction {
    Upward,
    Downward,
    Leftward,
    Rightward,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Steps(pub u8);

impl std::fmt::Display for Steps {
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

impl Into<u8> for Direction {
    fn into(self) -> u8 {
        match self {
            Direction::Upward => 4,
            Direction::Downward => 2,
            Direction::Leftward => 4,
            Direction::Rightward => 2,
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

/// Generate the number of steps that the player should move.
///
/// ## Note
/// The implementation rule is:
/// - The range of the first step is 2 to 4.
/// - The sum of steps is 5 to 7.
/// - The second turn's steps can be calculated by `sum - first_step`.
pub(crate) fn gen_player_steps() -> (Steps, Steps) {
    let mut rng = rand::thread_rng();

    // SAFETY: unwraps are safe as the slices aren't empty.
    let first_step: u8 = *[2, 3, 4].choose(&mut rng).unwrap();
    let sum: u8 = *[5, 6, 7].choose(&mut rng).unwrap();

    (Steps(first_step), Steps(sum))
}

/// Generate the number of steps that the enemies will move.
///
/// ## Note
/// - The range of steps is 1 to 3.
/// - (1, 1) or (3, 3) won't happen.
pub(crate) fn gen_enemies_steps() -> Vec<(Steps, Steps)> {
    let mut rng = rand::thread_rng();

    let mut steps = Vec::new();
    while steps.len() < 2 {
        // SAFETY: unwraps are safe as the slices aren't empty.
        let steps_1: u8 = *[1, 2, 3].choose(&mut rng).unwrap();
        let steps_2: u8 = *[1, 2, 3].choose(&mut rng).unwrap();
        if (steps_1, steps_2) != (1, 1) && (steps_1, steps_2) != (3, 3) {
            steps.push((Steps(steps_1), Steps(steps_2)));
        }
    }

    steps
}

/// Generate enemies' positions.
pub(crate) fn gen_enemies_positions(
    steps: (Steps, Steps),
    east_and_west: bool,
) -> (Position, Position) {
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
