use crate::gamble_f::{GambleAction, GambleF};
use rand::random; // 0.8.4
use scottish_names::{first_name, surname, Sex}; // 0.2.2
use titlecase::titlecase; // 1.1.0

#[derive(Debug)]
pub struct Gambler {
    name: String,
    money: usize,
    bet: usize,
    flips: usize,
    peak_money: usize,
    successes: usize,
    failures: usize,
    gamble_f: GambleF,
}

impl Gambler {
    pub fn new(money: usize, code: &str) -> Self {
        Self {
            name: format!(
                "{} {}",
                titlecase(first_name(if random() { Sex::Male } else { Sex::Female })),
                titlecase(surname())
            ),
            money,
            bet: 0,
            flips: 0,
            peak_money: money,
            successes: 0,
            failures: 0,
            gamble_f: GambleF::new(code),
        }
    }

    pub fn flip(&mut self) -> bool {
        if self.bet > self.money {
            self.money = 0;
            return false;
        }

        let success: bool = random();

        if success {
            self.money += self.bet;
            self.successes += 1;
        } else {
            self.money -= self.bet;
            self.failures += 1;
        }

        success
    }

    pub fn gamble(&mut self) -> String {
        while self.money > 0 {
            match self.tick() {
                Some(GambleAction::Bet(b)) => {
                    self.bet = b;
                    self.flip();
                    let result = self.flip();
                    self.gamble_f.gamble_result(result);
                }
                Some(GambleAction::RequestBalance) => self.gamble_f.balance(self.money),
                None => (),
            }
        }
        self.gamble_f.code_string()
    }

    pub fn tick(&mut self) -> Option<GambleAction> {
        self.gamble_f.tick()
    }
}

impl std::fmt::Display for Gambler {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Name: {}
Money: {}
Killer bet: {}
Flips survived: {}
Successful flips: {}
Unsuccessful flips: {}
Peak money: {}
Code: {}",
            self.name,
            self.money,
            self.bet,
            self.flips,
            self.successes,
            self.failures,
            self.peak_money,
            self.gamble_f.code_string(),
        )
    }
}
