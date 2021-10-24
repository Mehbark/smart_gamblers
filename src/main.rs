#![warn(clippy::all, clippy::pedantic)]

mod gambler;
use gambler::Gambler;

use markov::Chain;
mod gamble_f;

fn main() {
    let mut test = Gambler::new(10, vec!['+', '[', '.', '+', ']']);
    test.gamble();
    println!("{:#?}", test);
}

// TODO:
// Basically gamble_sim where the gamble_f code is generated via markov chain,
// and the code of the most succesful gamblers is added to the chain.
// Main problem is making a bunch of decent gambling algorithms is brainf to start off lol