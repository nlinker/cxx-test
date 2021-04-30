use cxx_test::game::Game;

fn main() {
    let game = Game::new();
    let ints = game.get_ints();
    let pts = game.get_points();
    println!("ints = {:?}, pts = {:?}", ints, pts);
}
