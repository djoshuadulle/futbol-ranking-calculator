use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ClubStats {
    games_played: u8,
    win: u8,
    draw: u8,
    loss: u8,
    goals_for: u8,
    goals_against: u8,
    goal_differential: i8,
    points: u8,
}

impl Default for ClubStats {
    fn default() -> ClubStats {
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
}

// TODO: Create a struct MatchScore {
// a_team: String,
// b_team: String,
// a_score: u8,
// b_score: u8,
// }

impl ClubStats {
    fn update_ranking(&mut self) {
        self.goal_differential = self.goals_for as i8 - self.goals_against as i8;
        self.points = 3 * self.win + self.draw;
    }

    fn add_match_result(&mut self, goals_for: u8, goals_against: u8) {
        if goals_for > goals_against {
            self.win += 1;
        } else if goals_for == goals_against {
            self.draw += 1;
        } else {
            self.loss += 1;
        }

        self.games_played = self.win + self.draw + self.loss;
        self.goals_for += goals_for;
        self.goals_against += goals_against;

        self.update_ranking();
    }
}

fn add_match_to_standings(
    standings: &mut HashMap<String, ClubStats>,
    a_team: String,
    b_team: String,
    a_score: u8,
    b_score: u8,
) {
    let club_stat = standings.entry(a_team).or_insert(ClubStats::default());
    club_stat.add_match_result(a_score, b_score);

    let club_stat = standings.entry(b_team).or_insert(ClubStats::default());
    club_stat.add_match_result(b_score, a_score);
}

fn main() {
    let mut standings = HashMap::<String, ClubStats>::new();

    // TODO: fn open_scores_stream()
    // Q: Is there a way to cleanly get project root directory?
    // Create a path to the scores file
    let mut path = PathBuf::from(env::current_dir().unwrap());
    path.push("scores.csv");

    // Open the path in read-only mode, returns `io::Result<File>`, then pass
    // to a BufReader to create an iterator over each line
    let scores_stream = match File::open(&path.as_path()) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
        Ok(file) => BufReader::new(file).lines(),
    };

    for score in scores_stream {
        // TODO: fn parse_score_ln() -> MatchScore, let user know if input data is bad
        let data = score.unwrap();
        let data: Vec<&str> = data.split(',').collect();

        let a_team = data[0];
        let b_team = *data.get(1).unwrap();
        let a_score = data[2].parse::<u8>().unwrap(); // TODO: handle u8 conversion error
        let b_score: u8 = data.get(3).unwrap().parse().unwrap(); // TODO: handle u8 conversion error

        // TODO: If no score line parsing Err, add MatchScore
        add_match_to_standings(
            &mut standings,
            a_team.to_string(),
            b_team.to_string(),
            a_score,
            b_score,
        );
    }

    // TODO: fn rank_clubs()
    // TODO: override functions for comparing to compare
    // by points, then goal differential, then total goals
    let mut standings_vec: Vec<(&String, &ClubStats)> = standings.iter().collect();
    standings_vec.sort_by(|a, b| b.1.cmp(a.1));

    // TODO: fn write_standings_csv()
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut path = PathBuf::from(env::current_dir().unwrap());
    path.push("standings.csv");
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    file.write_all(String::from("Club, MP, W, D, L, GF, GA, GD, Pts\n").as_bytes())
        .ok();

    for val in standings_vec {
        let team_name = val.0;
        let club_stat = val.1;

        let entry = String::from(format!(
            "{},{},{},{},{},{},{},{},{}\n",
            team_name,
            club_stat.games_played,
            club_stat.win,
            club_stat.draw,
            club_stat.loss,
            club_stat.goals_for,
            club_stat.goals_against,
            club_stat.goal_differential,
            club_stat.points
        ));
        file.write_all(entry.as_bytes()).ok();
    }

    println!("Standings written to: {}", path.display());
}
