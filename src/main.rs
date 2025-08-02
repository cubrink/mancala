mod mancala;

fn main() {
    let state = mancala::State::new();
    println!("Hello, world!, {}", state.player());
}
