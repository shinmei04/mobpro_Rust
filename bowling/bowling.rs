#![cfg_attr(feature = "strict", deny(warnings))]

pub fn score(game: &str) -> u32 {
    let mut total = 0;
    let mut previous = 0;
    let mut strike_bonus = 0;
    let mut spare_bonus = false;

    for character in game.chars() {
        if let Some(pins) = character.to_digit(10) {
            total += pins;

            if strike_bonus > 0 {
                total += pins;
                strike_bonus -= 1;
            }

            previous = pins;

            if spare_bonus {
                total += pins;
                spare_bonus = false;
            }
        } else if character == '/' {
            let pins = 10 - previous;
            total += pins;
            spare_bonus = true;
        } else if character == '-' {
            previous = 0;
            spare_bonus = false;
            if strike_bonus > 0 {
                strike_bonus -= 1;
            }
        } else if character == 'X' {
            total += 10;
            strike_bonus = 2;
        }
    }
    total
}
