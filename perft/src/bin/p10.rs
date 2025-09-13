use mancala_lib::Mancala;
use perft::perft;

fn main() {
    let count = perft(&Mancala::default(), 10);
    println!("perft(10) = {count}");
}
