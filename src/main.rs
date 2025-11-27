use chess_turnament_manager::player;
use chess_turnament_manager::schedule;

fn main() {
    println!("Chess Tournament Manager");
    let players = chess_turnament_manager::add_players();
    schedule::generate_round_robin(players);
}
