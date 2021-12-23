use std::collections::HashMap;

struct DeterministicDie {
    max_val: u64,
    prev_val: u64,
}

impl DeterministicDie {
    fn new(max_val: u64) -> Self {
        Self {
            max_val,
            prev_val: 0,
        }
    }

    fn next(&mut self) -> u64 {
        self.prev_val += 1;
        if self.prev_val > self.max_val {
            self.prev_val = 1;
        }
        self.prev_val
    }
}

const DIRAC_ROLLS: [u8; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

pub fn run(lines: &[String]) -> (u64, u64) {
    let initial_player_1 = lines[0][28..].parse::<u64>().unwrap() - 1;
    let initial_player_2 = lines[1][28..].parse::<u64>().unwrap() - 1;

    let mut player_1 = initial_player_1;
    let mut player_2 = initial_player_2;

    let mut player_1_score = 0;
    let mut player_2_score = 0;

    let mut d100 = DeterministicDie::new(100);

    let mut i = 0;
    let part_1;
    loop {
        let roll = d100.next() + d100.next() + d100.next();
        i += 3;
        player_1 += roll;
        player_1 %= 10;
        player_1_score += player_1 + 1;
        if player_1_score >= 1000 {
            part_1 = player_2_score * i;
            break;
        }

        let roll = d100.next() + d100.next() + d100.next();
        i += 3;
        player_2 += roll;
        player_2 %= 10;
        player_2_score += player_2 + 1;
        if player_2_score >= 1000 {
            part_1 = player_1_score * i;
            break;
        }
    }

    let mut player_1_wins = 0;
    let mut player_2_wins = 0;

    // p1 p2 p1_score p2_score
    // type State = (u8, u8, u8, u8);

    let mut states = HashMap::new();
    states.insert((initial_player_1 as u8, initial_player_2 as u8, 0, 0), 1);

    while !states.is_empty() {
        let mut next_states = HashMap::new();
        for (state, frequency) in &states {
            for (roll1, roll1_frequency) in DIRAC_ROLLS.iter().enumerate() {
                let mut new_state = *state;
                new_state.0 += roll1 as u8;
                new_state.0 %= 10;
                new_state.2 += new_state.0 + 1;
                if new_state.2 >= 21 {
                    player_1_wins += (*roll1_frequency as u64) * (*frequency as u64);
                } else {
                    for (roll2, roll2_frequency) in DIRAC_ROLLS.iter().enumerate() {
                        let mut new_state = new_state;
                        new_state.1 += roll2 as u8;
                        new_state.1 %= 10;
                        new_state.3 += new_state.1 + 1;
                        if new_state.3 >= 21 {
                            player_2_wins += (*roll1_frequency as u64)
                                * (*roll2_frequency as u64)
                                * (*frequency as u64);
                        } else {
                            let num_new_universes = (*roll1_frequency as u64)
                                * (*roll2_frequency as u64)
                                * (*frequency as u64);
                            if num_new_universes > 0 {
                                *next_states.entry(new_state).or_insert(0) += num_new_universes;
                            }
                        }
                    }
                }
            }
        }
        states = next_states;
    }

    (part_1, player_1_wins.max(player_2_wins))
}
