// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

/*
 * The fees a transaction offers, read from a real eth_feeHistory answer
 * (publicnode, mainnet, 2026-10-05): the next block's base fee, the median
 * of the blocks' median tips, the cap twice the base and the tip, and no
 * fixed gwei level anywhere.
 */

use crate::fees::{fees, parse_fee_history, MIN_TIP_WEI};

const MAINNET: &[u8] = include_bytes!("../fixtures/fee-history-mainnet.json");
const GWEI: u128 = 1_000_000_000;

#[test]
fn a_real_answer_gives_the_next_base_fee_and_ten_tips() {
    let (base, tips) = parse_fee_history(MAINNET).expect("read");
    assert_eq!(tips.len(), 10);
    /* Under a gwei, as mainnet runs today. */
    assert!(base > 0 && base < GWEI, "{base}");
    let (tip, cap) = fees(base, tips);
    assert!((MIN_TIP_WEI..GWEI).contains(&tip), "a tip under a gwei: {tip}");
    assert_eq!(cap, 2 * base + tip);
}

#[test]
fn the_tip_is_the_median_and_never_below_the_floor() {
    assert_eq!(fees(100, vec![5 * GWEI, GWEI, 3 * GWEI]).0, 3 * GWEI);
    assert_eq!(fees(100, vec![GWEI, 3 * GWEI]).0, 2 * GWEI);
    assert_eq!(fees(100, vec![0, 0, 0]).0, MIN_TIP_WEI);
    assert_eq!(fees(100, vec![]).0, MIN_TIP_WEI);
    assert_eq!(fees(GWEI, vec![GWEI]).1, 3 * GWEI);
}

#[test]
fn an_error_or_a_short_answer_gives_no_fees() {
    assert_eq!(parse_fee_history(br#"{"id":5,"error":{"code":-32000,"message":"x"}}"#), None);
    assert_eq!(parse_fee_history(br#"{"id":5,"result":{"baseFeePerGas":[]}}"#), None);
    assert_eq!(parse_fee_history(br#"{"id":5,"result":{"reward":[["0x1"]]}}"#), None);
}

/* The review writes every digit: 0.0000005 ETH is not 0, and a fee cap is
 * never shown less than it is. */
#[test]
fn an_amount_is_written_with_every_digit() {
    use crate::exact::exact_text;
    assert_eq!(exact_text(500_000_000_000, 18), "0.0000005");
    assert_eq!(exact_text(1, 18), "0.000000000000000001");
    assert_eq!(exact_text(2_000_000_000_000_000_000, 18), "2");
    assert_eq!(exact_text(1_250_000, 6), "1.25");
    assert_eq!(exact_text(0, 6), "0");
    assert_eq!(exact_text(u128::MAX, 18), "340282366920938463463.374607431768211455");
}
