pub mod club_stats;
use crate::club_stats::ClubStats;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

struct MatchScore {
    a_team: String,
    b_team: String,
    a_score: u8,
    b_score: u8,
}

fn parse_score_ln(score_line: String) -> Result<MatchScore, String> {
    let data: Vec<&str> = score_line.split(',').collect();

    if data.len() < 4 {
        return Err("Input line in scores csv empty or too short.".to_string());
    }

    let a_team = data[0];
    let b_team = data[1];

    let a_score = data[2]
        .parse::<u8>()
        .or_else(|_err| Err("Bad value in scores csv.".to_string()))?;
    let b_score = data[3]
        .parse::<u8>()
        .or_else(|_err| Err("Bad value in scores csv.".to_string()))?;

    Ok(MatchScore {
        a_team: a_team.to_string(),
        b_team: b_team.to_string(),
        a_score: a_score,
        b_score: b_score,
    })
}

fn add_match_to_standings(standings: &mut HashMap<String, ClubStats>, match_score: MatchScore) {
    let club_stat = standings
        .entry(match_score.a_team)
        .or_insert(ClubStats::new());
    club_stat.add_match_result(match_score.a_score, match_score.b_score);

    let club_stat = standings
        .entry(match_score.b_team)
        .or_insert(ClubStats::new());
    club_stat.add_match_result(match_score.b_score, match_score.a_score);
}

fn rank_clubs(mut standings: HashMap<String, ClubStats>) -> Vec<(String, ClubStats)> {
    let mut standings_vec: Vec<(String, ClubStats)> = standings.drain().collect();
    standings_vec.sort_by(|b, a| {
        let a_stat = a.1;
        let b_stat: ClubStats = b.1;

        // compare by points
        if a_stat.points > b_stat.points {
            return Ordering::Greater;
        }
        if b_stat.points > a_stat.points {
            return Ordering::Less;
        }

        // then compare by goal differential
        if a_stat.goal_differential > b_stat.goal_differential {
            return Ordering::Greater;
        }
        if b_stat.goal_differential > a_stat.goal_differential {
            return Ordering::Less;
        }

        // then compare by total goals
        if a_stat.goals_for > b_stat.goals_for {
            return Ordering::Greater;
        }
        if b_stat.goals_for > a_stat.goals_for {
            return Ordering::Less;
        }

        Ordering::Equal
    });

    return standings_vec;
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let in_path = &args[1];
    println!("innnn::::: {in_path}");

    // TODO: fn open_scores_stream()
    // Q: Is there a way to cleanly get project root directory?
    // Create a path to the scores file
    let path = PathBuf::from(in_path);
    // path.push("scores_tie.csv");

    // Open the path in read-only mode, returns `io::Result<File>`, then pass
    // to a BufReader to create an iterator over each line
    let scores_iter = match File::open(&path.as_path()) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
        Ok(file) => BufReader::new(file).lines(),
    };

    let mut standings = HashMap::<String, ClubStats>::new();

    for score_line in scores_iter {
        let match_score = parse_score_ln(score_line.unwrap())?;
        add_match_to_standings(&mut standings, match_score);
    }

    let standings_vec = rank_clubs(standings);

    // TODO: fn write_standings_csv()
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut path = PathBuf::from(env::current_dir().unwrap());
    path.push("standings.csv");
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(error) => panic!("Couldn't create {}: {}", path.display(), error),
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
    Ok(())
}
