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

//! app_id held to nonos_linux_policy's pinned vector in C1b's layout (PLAN
//! v1.1), and A's bit order: bit b is bit b % 8 of byte b / 8.

use nonos_proof_abi::{allows, app_id};

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// C1b's inputs, laid out as plan/30 places them.
fn c1b_policy() -> [u8; 256] {
    let mut p = [0u8; 256];
    p[..8].copy_from_slice(b"NLXPOL01");
    p[8..10].copy_from_slice(&1u16.to_le_bytes());
    p[16..48].copy_from_slice(&[0x11; 32]);
    p[48..56].copy_from_slice(&0x21u64.to_le_bytes());
    p[56..88].copy_from_slice(&[0x22; 32]);
    p[88..120].copy_from_slice(&[0x33; 32]);
    p[120..152].copy_from_slice(&[0x44; 32]);
    for b in [0usize, 1, 231, 511, 512] {
        p[152 + b / 8] |= 1 << (b % 8);
    }
    p
}

#[test]
fn app_id_is_the_policy_crates_pinned_vector() {
    let p = c1b_policy();
    let policy_hash = blake3::hash(&[&b"NONOS.linux.policy.v1"[..], &p].concat());
    assert_eq!(
        hex(policy_hash.as_bytes()),
        "b53947e2b646947216fc959c5d5ebef03c97faf72a0970500dd964f3f586b596"
    );
    assert_eq!(
        hex(&app_id(&p, &[0x11; 32])),
        "6a540c105587fd8759391f107f2caa5279a908420821a2c30c778bdca7dca2fa"
    );
    let mut set = [0u8; 65];
    set.copy_from_slice(&p[152..217]);
    let on: Vec<usize> = (0..513).filter(|&b| allows(&set, b)).collect();
    assert_eq!(on, [0, 1, 231, 511, 512]);
}
