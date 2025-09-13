use mancala_lib::Mancala;
use perft::perft;

fn main() {
    let count = perft(&Mancala::default(), 8);
    println!("perft(8) = {count}");
}
