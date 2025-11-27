use crate::player::Player;

pub fn generate_round_robin(mut players: Vec<Player>) -> Vec<Vec<(Player, Player)>> {
    // Add BYE if odd number of players
    if players.len() % 2 != 0 {
        players.push(Player::new("BYE".into(), None));
    }

    let n = players.len();
    let rounds = n - 1;
    let mut schedule = Vec::new();

    // We will rotate players[1..]
    for _round in 0..rounds {
        let mut pairings = Vec::new();

        for i in 0..n/2 {
            let white = players[i].clone();
            let black = players[n - 1 - i].clone();
            pairings.push((white, black));
        }

        schedule.push(pairings);

        // Rotate players except for the first element
        let fixed = players[0].clone();
        let mut rotating = players[1..].to_vec();
        rotating.rotate_right(1);

        players = std::iter::once(fixed).chain(rotating).collect();
    }

    schedule
}
