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

use nonos_eth_rpc::{fees, parse_fee_history, MIN_TIP_WEI};

const ANSWER: &[u8] = br#"{"jsonrpc":"2.0","id":3,"result":{"oldestBlock":"0x10",
"baseFeePerGas":["0x3b9aca00","0x3b9aca00","0x77359400"],"gasUsedRatio":[0.5,0.4],
"reward":[["0x5f5e100"],["0x2faf080"],["0xbebc200"]]}}"#;

#[test]
fn the_next_base_fee_and_the_median_tip_set_the_cap() {
    let (next, tips) = parse_fee_history(ANSWER).unwrap();
    assert_eq!(next, 2_000_000_000, "the last base fee is the next block's");
    assert_eq!(tips, vec![100_000_000, 50_000_000, 200_000_000]);
    let (tip, cap) = fees(next, tips);
    assert_eq!((tip, cap), (100_000_000, 4_100_000_000));
}

#[test]
fn an_error_or_no_tips_is_handled_and_the_tip_has_a_floor() {
    assert_eq!(parse_fee_history(br#"{"error":{"code":-32000},"baseFeePerGas":["0x1"]}"#), None);
    assert_eq!(parse_fee_history(br#"{"result":{"reward":[]}}"#), None, "no base fee");
    assert_eq!(fees(7, vec![]), (MIN_TIP_WEI, 14 + MIN_TIP_WEI));
    assert_eq!(fees(0, vec![MIN_TIP_WEI, 3 * MIN_TIP_WEI]).0, 2 * MIN_TIP_WEI, "an even count");
}
