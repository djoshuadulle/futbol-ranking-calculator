#[derive(Debug, Copy, Clone)]
pub struct ClubStats {
    pub games_played: u8,
    pub win: u8,
    pub draw: u8,
    pub loss: u8,
    pub goals_for: u8,
    pub goals_against: u8,
    pub goal_differential: i8,
    pub points: u8,
}

impl ClubStats {
    pub fn new() -> ClubStats {
        ClubStats {
            games_played: 0,
            win: 0,
            draw: 0,
            loss: 0,
            goals_for: 0,
            goals_against: 0,
            goal_differential: 0,
            points: 0,
        }
    }

    pub fn add_match_result(&mut self, goals_for: u8, goals_against: u8) {
        if goals_for > goals_against {
            self.win += 1;
        } else if goals_for == goals_against {
            self.draw += 1;
        } else {
            self.loss += 1;
        }

        self.goals_for += goals_for;
        self.goals_against += goals_against;

        self.update_stats();
    }

    fn update_stats(&mut self) {
        self.games_played = self.win + self.draw + self.loss;
        self.goal_differential = self.goals_for as i8 - self.goals_against as i8;
        self.points = 3 * self.win + self.draw;
    }
}
