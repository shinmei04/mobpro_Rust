#![cfg_attr(feature = "strict", deny(warnings))]

use hiker::score;
#[test]

fn cal_score() {
    let game = "X|34|--|--|--|--|--|--|--|--||";

    assert_eq!(score(game), 24);
}
