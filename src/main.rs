use chess_turnament_manager::schedule;

fn main() {
    let players = chess_turnament_manager::add_players();
    schedule::generate_round_robin(players);
}
