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

use nonos_nox_config::{domain_answer_matches, SETTLEMENT};

fn num(n: u64) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[24..].copy_from_slice(&n.to_be_bytes());
    w
}

fn text(s: &str) -> Vec<u8> {
    let mut out = num(s.len() as u64).to_vec();
    out.extend_from_slice(s.as_bytes());
    out.resize(32 + s.len().div_ceil(32) * 32, 0);
    out
}

/* eip712Domain()'s return as solc encodes it. */
fn answer(name: &str, version: &str, chain: u64, at: [u8; 20]) -> Vec<u8> {
    let mut fields = [0u8; 32];
    fields[0] = 0x0f;
    let (n, v) = (text(name), text(version));
    let mut contract = [0u8; 32];
    contract[12..].copy_from_slice(&at);
    let mut out = fields.to_vec();
    out.extend_from_slice(&num(224));
    out.extend_from_slice(&num(224 + n.len() as u64));
    out.extend_from_slice(&num(chain));
    out.extend_from_slice(&contract);
    out.extend_from_slice(&[0; 32]);
    out.extend_from_slice(&num(224 + (n.len() + v.len()) as u64));
    out.extend(n);
    out.extend(v);
    out.extend_from_slice(&num(0));
    out
}

#[test]
fn only_the_configs_own_domain_matches() {
    let at = [7u8; 20];
    let good = answer("NONOS Settlement", "1", 1, at);
    assert!(domain_answer_matches(&SETTLEMENT, 1, &at, &good));
    assert!(!domain_answer_matches(&SETTLEMENT, 11_155_111, &at, &good), "another chain");
    assert!(!domain_answer_matches(&SETTLEMENT, 1, &[8; 20], &good), "another contract");
    assert!(!domain_answer_matches(&SETTLEMENT, 1, &at, &answer("NONOS Bond", "1", 1, at)));
    assert!(!domain_answer_matches(&SETTLEMENT, 1, &at, &answer("NONOS Settlement", "2", 1, at)));
    let mut salted = good.clone();
    salted[5 * 32] = 1;
    assert!(!domain_answer_matches(&SETTLEMENT, 1, &at, &salted), "a salt");
    let mut fields = good.clone();
    fields[0] = 0x1f;
    assert!(!domain_answer_matches(&SETTLEMENT, 1, &at, &fields), "a salt field");
    let mut extended = good.clone();
    *extended.last_mut().unwrap() = 1;
    assert!(!domain_answer_matches(&SETTLEMENT, 1, &at, &extended), "extensions");
    for n in 0..good.len() {
        assert!(!domain_answer_matches(&SETTLEMENT, 1, &at, &good[..n]), "{n} bytes");
    }
    let long = [good.as_slice(), &[0; 32]].concat();
    assert!(!domain_answer_matches(&SETTLEMENT, 1, &at, &long), "a trailing word");
}
