use cxx_test::game::Game;

fn main() {
    let game = Game::new();
    game.debug();

    let ints = game.get_ints();
    let pts = game.get_points();
    println!("ints = {:?}", ints);
    println!("pts = {:?}", pts);
}
