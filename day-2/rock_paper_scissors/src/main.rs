use std::{fs::File, io::{BufReader, BufRead, self}, str::FromStr};


#[derive(Debug)]
enum HandShape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
enum GameResult {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl FromStr for HandShape {
    type Err = ();
    fn from_str(input: &str) -> Result<HandShape, Self::Err> {
        match input {
            "A"  => Ok(HandShape::Rock),
            "B"  => Ok(HandShape::Paper),
            "C"  => Ok(HandShape::Scissors),
            _      => Err(()),
        }
    }
}

impl FromStr for GameResult {
    type Err = ();
    fn from_str(input: &str) -> Result<GameResult, Self::Err> {
        match input {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _      => Err(()),
        }
    }
}

fn did_win(me: &HandShape, opponent: &HandShape) -> GameResult {
    match me {
        HandShape::Rock => {
            match opponent {
                HandShape::Rock => {
                    return GameResult::Draw;
                }
                HandShape::Paper => {
                    return GameResult::Lose;
                },
                HandShape::Scissors => {
                    return GameResult::Win;
                }
            }
        },
        HandShape::Paper => {
            match opponent {
                HandShape::Rock => {
                    return GameResult::Win;
                }
                HandShape::Paper => {
                    return GameResult::Draw;
                },
                HandShape::Scissors => {
                    return GameResult::Lose;
                }
            }
        },
        HandShape::Scissors => {
            match opponent {
                HandShape::Rock => {
                    return GameResult::Lose;
                }
                HandShape::Paper => {
                    return GameResult::Win;
                },
                HandShape::Scissors => {
                    return GameResult::Draw;
                }
            }
        }
    }
}

fn get_my_hand_shape(opponent_hand_shape: &HandShape, game_result: &GameResult) -> HandShape {
    match opponent_hand_shape {
        HandShape::Rock => {
            match game_result {
                GameResult::Win => {
                    return HandShape::Paper;
                },
                GameResult::Draw => {
                    return HandShape::Rock;
                },
                GameResult::Lose => {
                    return HandShape::Scissors;
                },
            }
        }
        HandShape::Paper => {
            match game_result {
                GameResult::Win => {
                    return HandShape::Scissors;
                },
                GameResult::Draw => {
                    return HandShape::Paper;
                },
                GameResult::Lose => {
                    return HandShape::Rock;
                },
            }
        },
        HandShape::Scissors => {
            match game_result {
                GameResult::Win => {
                    return HandShape::Rock;
                },
                GameResult::Draw => {
                    return HandShape::Scissors;
                },
                GameResult::Lose => {
                    return HandShape::Paper;
                },
            }
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    
    let mut score = 0;

    for line_res in reader.lines() {
        let line = line_res?;
        let mut choices = line.split_whitespace();
        let opponent_hand_shape = HandShape::from_str(choices.next()?)?;
        let game_result = GameResult::from_str(choices.next()?)?;

        let my_hand_shape = get_my_hand_shape(&opponent_hand_shape, &game_result);

        println!("{:?} vs {:?} -> {:?}", my_hand_shape, opponent_hand_shape, game_result);

        score = score + (game_result as i32) + (my_hand_shape as i32);
    }

    println!("{}", score);

    Ok(())
}
