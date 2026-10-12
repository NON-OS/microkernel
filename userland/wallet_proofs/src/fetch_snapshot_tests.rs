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

//! A batch reply of the shape the RPC hosts answer parses into every field.

use super::*;

#[test]
fn mainnet_reply_parses_every_field() {
    let _held = hold();
    chain::pick(false);
    let mut parts = common();
    parts.push(quantity(11, "2a"));
    parts.push(quantity(12, "3"));
    /* getProtocolStats: total staked is word 0, rewards 3, emission 5. */
    let stats: String = [1_000_000, 0, 0, 9, 0, 5].iter().map(|&n| word(n)).collect();
    parts.push(format!("{{\"jsonrpc\":\"2.0\",\"id\":13,\"result\":\"0x{stats}\"}}"));
    let info = format!("{}{}{}", word(0), word(0), word(2));
    parts.push(format!("{{\"jsonrpc\":\"2.0\",\"id\":14,\"result\":\"0x{info}\"}}"));
    let s = parse_snapshot(&reply(&parts));
    assert_eq!(s.eth_balance, Some(q(ETH)));
    assert_eq!(s.nox_balance, Some(q(NOX)));
    assert_eq!(s.usdc_balance, Some(q(USDC)));
    assert_eq!(s.nonce, Some(7));
    assert_eq!(s.fee, Some(1_000_000_000));
    assert_eq!(s.claimable, Some(q("2a")));
    assert_eq!(s.positions, Some(3));
    assert_eq!(s.passes, Some(2));
    let stats = s.stats.expect("stats");
    assert_eq!(stats.total, q("f4240"));
    assert_eq!(stats.rewards, q("9"));
}

#[test]
fn sepolia_reply_parses_and_asks_nothing_of_staking() {
    let _held = hold();
    chain::pick(true);
    let body = snapshot_request(&ADDR);
    let text = String::from_utf8(body).unwrap();
    assert!(!text.contains("\"id\":11"));
    assert!(text.contains("3e5249a65ca513d5e11260222e0d26f46b465d36"));
    let s = parse_snapshot(&reply(&common()));
    assert_eq!(s.eth_balance, Some(q(ETH)));
    assert_eq!(s.nox_balance, Some(q(NOX)));
    assert_eq!(s.usdc_balance, Some(q(USDC)));
    assert_eq!(s.nonce, Some(7));
    assert_eq!(s.fee, Some(1_000_000_000));
    assert_eq!(s.claimable, None);
    assert!(s.stats.is_none());
}
