use crate::player::Player;
use std::io::{self, Write};

const BOLD: &str = "\x1b[1m";
const GREEN: &str = "\x1b[32m";
const ORANGE: &str = "\x1b[38;5;208m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";
const BYE_PLAYER: &str = "<bye>";

pub fn generate_round_robin(mut players: Vec<Player>) {
    let original_count = players.len();
    let has_bye = original_count % 2 == 1;
    
    if has_bye {
        players.push(Player::new(BYE_PLAYER.to_string(), None));
    }
    
    println!("{}Enter results: 1=white wins, 0=draw, -1=black wins{}\n", BOLD, RESET);
    
    let n = players.len();
    let mut opponent_scores: Vec<Vec<f32>> = vec![vec![]; n];
    
    for round in 0..n - 1 {
        println!("{}{}{}{}", BOLD, ORANGE, format!("Round {}:", round + 1), RESET);
        let mut matches = Vec::new();
        let mut board = 1;
        
        for i in 0..n / 2 {
            let (idx1, idx2) = (i, n - 1 - i);
            
            if is_bye(players[idx1].name.as_str()) {
                println!("{}{}{} {} gets a break", YELLOW, "●", RESET, players[idx2].name);
                opponent_scores[idx2].push(0.5);
            } else if is_bye(players[idx2].name.as_str()) {
                println!("{}{}{} {} gets a break", YELLOW, "●", RESET, players[idx1].name);
                opponent_scores[idx1].push(0.5);
            } else {
                let (white_idx, black_idx) = assign_colors(&players, idx1, idx2);
                println!(
                    "{}Board {}: {}{} (W){} vs {}{} (B){}",
                    GREEN, board, BOLD, players[white_idx].name, RESET,
                    players[black_idx].name, ORANGE, RESET
                );
                players[white_idx].whites += 1;
                players[black_idx].blacks += 1;
                matches.push((white_idx, black_idx));
                board += 1;
            }
        }
        
        for (white_idx, black_idx) in &matches {
            print!("Result for {} vs {} (1/0/-1): ", players[*white_idx].name, players[*black_idx].name);
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            process_result(&mut players, &mut opponent_scores, *white_idx, *black_idx, input.trim());
        }
        
        if round < n - 2 {
            let last = players.pop().unwrap();
            players.insert(1, last);
        }
    }
    
    for (i, player) in players.iter_mut().enumerate() {
        if !is_bye(player.name.as_str()) {
            player.buchholz = opponent_scores[i].iter().sum();
        }
    }
    
    display_scoreboard(&players, original_count);
}

fn is_bye(name: &str) -> bool {
    name == BYE_PLAYER
}

fn assign_colors(players: &[Player], idx1: usize, idx2: usize) -> (usize, usize) {
    let (w1, w2, b1, b2) = (players[idx1].whites, players[idx2].whites, players[idx1].blacks, players[idx2].blacks);
    
    if w1 > w2 {
        (idx2, idx1)
    } else if w2 > w1 {
        (idx1, idx2)
    } else if b1 > b2 {
        (idx1, idx2)
    } else {
        (idx2, idx1)
    }
}

fn process_result(players: &mut [Player], opponent_scores: &mut [Vec<f32>], white_idx: usize, black_idx: usize, input: &str) {
    let (white_points, black_points) = match input.parse::<i32>() {
        Ok(1) => {
            players[white_idx].wins += 1;
            (players[black_idx].points(), players[white_idx].points())
        }
        Ok(-1) => {
            players[black_idx].wins += 1;
            (players[black_idx].points(), players[white_idx].points())
        }
        _ => {
            players[white_idx].draws += 1;
            players[black_idx].draws += 1;
            (players[black_idx].points(), players[white_idx].points())
        }
    };
    
    opponent_scores[white_idx].push(white_points);
    opponent_scores[black_idx].push(black_points);
}

fn display_scoreboard(players: &[Player], original_count: usize) {
    let mut scores: Vec<_> = players.iter().filter(|p| !is_bye(p.name.as_str())).collect();
    
    scores.sort_by(|a, b| {
        b.points()
            .partial_cmp(&a.points())
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.buchholz.partial_cmp(&a.buchholz).unwrap_or(std::cmp::Ordering::Equal))
            .then_with(|| b.wins.cmp(&a.wins))
    });
    
    let games = original_count - 1;
    println!("\n{}{}{}", BOLD, GREEN, "\n=== FINAL STANDINGS ===");
    println!("{}({} games per player - Round Robin)", RESET, games);
    println!("{}Rank | Player{:<15} | Points | Buchholz | W  D  L{}", RESET, "", "");
    println!("{}{}", GREEN, "─".repeat(72));
    
    for (rank, player) in scores.iter().enumerate() {
        println!(
            "{}{}  {:2}. | {:<20} | {:5.1} | {:8.1} | {} {} {} {}",
            ORANGE, RESET, rank + 1, player.name, player.points(), player.buchholz,
            player.wins, player.draws, player.losses, RESET
        );
    }
    
    println!("{}{}{}\n{}Tiebreaker: Points → Buchholz → Wins{}\n", GREEN, "─".repeat(72), RESET, BOLD, RESET);
}
