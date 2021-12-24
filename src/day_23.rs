use std::cmp::{max, min, Ordering};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn from_char(c: char) -> Amphipod {
        use Amphipod::*;
        match c {
            'A' => A,
            'B' => B,
            'C' => C,
            'D' => D,
            _ => unreachable!(),
        }
    }

    fn energy(&self) -> u64 {
        use Amphipod::*;
        match self {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
        }
    }

    fn desired_room(&self) -> usize {
        use Amphipod::*;
        match self {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
        }
    }

    fn desired_amphipod(room_index: usize) -> Self {
        use Amphipod::*;
        match room_index {
            0 => A,
            1 => B,
            2 => C,
            3 => D,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct State {
    hall: [Option<Amphipod>; 11],
    rooms: [[Option<Amphipod>; 4]; 4],
    total_energy: u64,
}

impl State {
    fn from_front_back_1(front: &str, back: &str) -> State {
        let room_1 = [
            Some(Amphipod::from_char(front.chars().nth(3).unwrap())),
            Some(Amphipod::from_char(back.chars().nth(3).unwrap())),
            None,
            None,
        ];
        let room_2 = [
            Some(Amphipod::from_char(front.chars().nth(5).unwrap())),
            Some(Amphipod::from_char(back.chars().nth(5).unwrap())),
            None,
            None,
        ];
        let room_3 = [
            Some(Amphipod::from_char(front.chars().nth(7).unwrap())),
            Some(Amphipod::from_char(back.chars().nth(7).unwrap())),
            None,
            None,
        ];
        let room_4 = [
            Some(Amphipod::from_char(front.chars().nth(9).unwrap())),
            Some(Amphipod::from_char(back.chars().nth(9).unwrap())),
            None,
            None,
        ];
        State {
            hall: [
                None, None, None, None, None, None, None, None, None, None, None,
            ],
            rooms: [room_1, room_2, room_3, room_4],
            total_energy: 0,
        }
    }

    fn from_front_back_2(front: &str, back: &str) -> State {
        let room_1 = [
            Some(Amphipod::from_char(front.chars().nth(3).unwrap())),
            Some(Amphipod::D),
            Some(Amphipod::D),
            Some(Amphipod::from_char(back.chars().nth(3).unwrap())),
        ];
        let room_2 = [
            Some(Amphipod::from_char(front.chars().nth(5).unwrap())),
            Some(Amphipod::C),
            Some(Amphipod::B),
            Some(Amphipod::from_char(back.chars().nth(5).unwrap())),
        ];
        let room_3 = [
            Some(Amphipod::from_char(front.chars().nth(7).unwrap())),
            Some(Amphipod::B),
            Some(Amphipod::A),
            Some(Amphipod::from_char(back.chars().nth(7).unwrap())),
        ];
        let room_4 = [
            Some(Amphipod::from_char(front.chars().nth(9).unwrap())),
            Some(Amphipod::A),
            Some(Amphipod::C),
            Some(Amphipod::from_char(back.chars().nth(9).unwrap())),
        ];
        State {
            hall: [
                None, None, None, None, None, None, None, None, None, None, None,
            ],
            rooms: [room_1, room_2, room_3, room_4],
            total_energy: 0,
        }
    }

    fn is_done(&self, depth: usize) -> bool {
        self.hall.iter().all(|pos| pos.is_none())
            && self.rooms.iter().enumerate().all(|(room_index, room)| {
                room.iter().enumerate().all(|(i, pos)| {
                    if i < depth {
                        match pos {
                            Some(pod) => *pod == Amphipod::desired_amphipod(room_index),
                            None => false,
                        }
                    } else {
                        true
                    }
                })
            })
    }

    fn energy_needed(&self, depth: usize) -> u64 {
        if self.is_done(depth) {
            self.total_energy
        } else {
            self.possible_next_states(depth)
                .iter()
                .map(|state| state.energy_needed(depth))
                .min()
                .unwrap_or(u64::MAX)
        }
    }

    fn get(&self, position: Position) -> Option<Amphipod> {
        match position {
            Position::Hall(index) => *self.hall.get(index)?,
            Position::Room(room_index, room_place) => {
                *self.rooms.get(room_index)?.get(room_place)?
            }
        }
    }

    fn is_room_ready(&self, room_index: usize) -> bool {
        let room = self.rooms[room_index];
        room.iter().all(|room_pos| {
            room_pos.is_none() || *room_pos == Some(Amphipod::desired_amphipod(room_index))
        })
    }

    fn possible_next_states(&self, depth: usize) -> Vec<State> {
        // greedy: try to move into room
        for (i, space) in self.hall.iter().enumerate() {
            if let Some(pod) = space {
                if self.is_room_ready(pod.desired_room()) {
                    let mut room_depth = depth - 1;
                    while self.rooms[pod.desired_room()][room_depth].is_some() {
                        room_depth -= 1;
                    }
                    let destination = Position::Room(pod.desired_room(), room_depth);
                    let start = Position::Hall(i);
                    if !self.is_possible_move(start, destination) {
                        continue;
                    }

                    let mut hall = self.hall;
                    hall[i] = None;
                    let mut rooms = self.rooms;
                    rooms[pod.desired_room()][room_depth] = Some(*pod);
                    let energy_needed = Position::distance(start, destination) * pod.energy();
                    let next_state = State {
                        hall,
                        rooms,
                        total_energy: self.total_energy + energy_needed,
                    };
                    return vec![next_state];
                }
            }
        }

        let mut result = Vec::new();

        for i in 0..4 {
            if self.is_room_ready(i) {
                continue;
            }
            let mut start = 0;
            while self.get(Position::Room(i, start)).is_none() {
                start += 1;
                if start == depth {
                    continue;
                }
            }

            let start_pos = Position::Room(i, start);
            for destination in POSSIBLE_HALL_STOPS {
                if self.is_possible_move(start_pos, destination) {
                    let pod = self.get(start_pos).unwrap();
                    let hall_index = match destination {
                        Position::Hall(i) => i,
                        _ => unreachable!(),
                    };

                    let mut rooms = self.rooms;
                    rooms[i][start] = None;

                    let mut hall = self.hall;
                    hall[hall_index] = Some(pod);

                    let energy_needed = Position::distance(start_pos, destination) * pod.energy();
                    result.push(State {
                        hall,
                        rooms,
                        total_energy: self.total_energy + energy_needed,
                    });
                }
            }
        }

        result
    }

    fn is_possible_move(&self, start: Position, end: Position) -> bool {
        for position in Position::path(start, end) {
            if self.get(position).is_some() {
                return false;
            }
        }
        true
    }
}

const POSSIBLE_HALL_STOPS: [Position; 7] = [
    Position::Hall(0),
    Position::Hall(1),
    Position::Hall(3),
    Position::Hall(5),
    Position::Hall(7),
    Position::Hall(9),
    Position::Hall(10),
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Position {
    Hall(usize),
    Room(usize, usize),
}

impl Position {
    fn distance(p1: Self, p2: Self) -> u64 {
        use Position::*;
        let (p1_hall, p1_room) = match p1 {
            Hall(index) => (index, 0),
            Room(index, room) => (Self::hall_index(index), room + 1),
        };
        let (p2_hall, p2_room) = match p2 {
            Hall(index) => (index, 0),
            Room(index, room) => (Self::hall_index(index), room + 1),
        };
        let hall_dist = max(p1_hall, p2_hall) - min(p1_hall, p2_hall);
        (hall_dist + p1_room + p2_room) as u64
    }

    fn hall_index(room_index: usize) -> usize {
        2 + 2 * room_index
    }

    fn path(mut start: Self, end: Self) -> Vec<Position> {
        let mut result = Vec::new();
        while let Some(pos) = Self::next_in_path(start, end) {
            start = pos;
            result.push(pos);
        }
        result
    }

    fn next_in_path(start: Self, end: Self) -> Option<Position> {
        if start == end {
            return None;
        }

        match start {
            Position::Room(room_index, room_depth) => match end {
                Position::Room(end_room_index, _) => {
                    if end_room_index == room_index {
                        Some(Position::Room(room_index, room_depth + 1))
                    } else {
                        match room_depth {
                            0 => Some(Position::Hall(Position::hall_index(room_index))),
                            other => Some(Position::Room(room_index, other - 1)),
                        }
                    }
                }
                Position::Hall(_) => match room_depth {
                    0 => Some(Position::Hall(Position::hall_index(room_index))),
                    other => Some(Position::Room(room_index, other - 1)),
                },
            },

            Position::Hall(hall_index) => {
                let target_hall_index = match end {
                    Position::Room(room_index, _) => Position::hall_index(room_index),
                    Position::Hall(hall_index) => hall_index,
                };
                Some(match hall_index.cmp(&target_hall_index) {
                    Ordering::Equal => match end {
                        Position::Room(room_index, _) => Position::Room(room_index, 0),
                        _ => unreachable!(),
                    },
                    Ordering::Less => Position::Hall(hall_index + 1),
                    Ordering::Greater => Position::Hall(hall_index - 1),
                })
            }
        }
    }
}

pub fn run(lines: &[String]) -> (u64, u64) {
    let front = &lines[2];
    let back = &lines[3];

    let starting_state_1 = State::from_front_back_1(front, back);
    let part_1 = starting_state_1.energy_needed(2);

    let starting_state_2 = State::from_front_back_2(front, back);
    let part_2 = starting_state_2.energy_needed(4);

    (part_1, part_2)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_1() {
        let input = vec![
            String::from("#############"),
            String::from("#...........#"),
            String::from("###B#C#B#D###"),
            String::from("  #A#D#C#A#"),
            String::from("  #########"),
        ];

        let result = run(&input);

        assert_eq!(result, (12521, 44169));
    }
}
