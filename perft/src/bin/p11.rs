use mancala_lib::Mancala;
use perft::perft;

fn main() {
    let count = perft(&Mancala::default(), 11);
    println!("perft(11) = {count}");
}
