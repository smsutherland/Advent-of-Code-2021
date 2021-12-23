#[derive(Clone, Copy)]
enum Status {
    Marked(u64),
    Unmarked(u64),
}

struct Board {
    nums: Vec<Vec<Status>>,
}

impl Board {
    fn new(nums: Vec<Vec<Status>>) -> Board {
        Board { nums }
    }

    fn mark_num(&mut self, num: u64) -> bool {
        'breakout: for (y, row) in self.nums.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if let Status::Unmarked(val) = value {
                    if *val == num {
                        self.nums[y][x] = Status::Marked(*val);
                        break 'breakout;
                    }
                }
            }
        }

        self.won()
    }

    fn won(&self) -> bool {
        // check rows
        for row in &self.nums {
            if Self::check_group(row) {
                return true;
            }
        }

        // check cols
        for x in 0..5 {
            if Self::check_group(&self.generate_col(x)) {
                return true;
            }
        }

        // check main diag
        let mut main_diag = Vec::new();
        for xy in 0..5 {
            main_diag.push(self.nums[xy][xy]);
        }
        if Self::check_group(&main_diag) {
            return true;
        }

        // check off diag
        let mut off_diag = Vec::new();
        for xy in 0..5 {
            off_diag.push(self.nums[4 - xy][xy]);
        }
        if Self::check_group(&off_diag) {
            return true;
        }

        false
    }

    fn generate_col(&self, x: usize) -> Vec<Status> {
        let mut result = Vec::new();

        for i in 0..5 {
            result.push(self.nums[i][x]);
        }

        result
    }

    fn check_group(vals: &[Status]) -> bool {
        for val in vals {
            if let Status::Unmarked(_) = val {
                return false;
            }
        }
        true
    }

    fn score_part(&self) -> u64 {
        let mut result = 0;
        for row in &self.nums {
            for value in row {
                if let Status::Unmarked(val) = value {
                    result += val;
                }
            }
        }
        result
    }
}

pub fn run(lines: &[String]) -> (u64, u64) {
    let order: Vec<u64> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();

    let mut lines = lines.iter().skip(1);

    let mut boards: Vec<Board> = Vec::new();
    while let Some(_) = lines.next() {
        let mut board: Vec<Vec<Status>> = Vec::new();
        for _ in 0..5 {
            board.push(
                lines
                    .next()
                    .unwrap()
                    .split(' ')
                    .filter(|x| x.is_empty())
                    .map(|x| Status::Unmarked(x.parse().unwrap()))
                    .collect(),
            );
        }
        boards.push(Board::new(board));
    }

    let mut first_winner = None;
    let mut last_winner = None;
    let mut first_win_num = 0;
    let mut last_win_num = 0;

    let mut winners = vec![false; boards.len()];
    let mut num_winners = 0;

    'loop2: for num in order {
        for (i, board) in boards.iter_mut().enumerate() {
            if board.mark_num(num) {
                if first_winner == None {
                    first_win_num = num;
                    first_winner = Some(i);
                }

                if !winners[i] {
                    winners[i] = true;
                    num_winners += 1;
                }

                if num_winners == winners.len() {
                    last_win_num = num;
                    last_winner = Some(i);
                    break 'loop2;
                }
            }
        }
    }

    let part_1 = boards[first_winner.unwrap()].score_part() * first_win_num;
    let part_2 = boards[last_winner.unwrap()].score_part() * last_win_num;

    (part_1, part_2)
}
