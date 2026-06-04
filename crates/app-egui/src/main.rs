fn main() {
    let game = omniciv_game::Game::initialize().unwrap();
    println!("{game:#?}");
}
