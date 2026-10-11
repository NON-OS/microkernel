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

//! app_id held to nonos_linux_policy's pinned vector, C1's v1 layout (C1b
//! re-pins it for v1.1), and A's bit order: bit b is bit b % 8 of byte b / 8.

use nonos_proof_abi::{allows, app_id};

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// C1's inputs, laid out as the plan places them.
fn c1_policy() -> [u8; 256] {
    let mut p = [0u8; 256];
    p[..8].copy_from_slice(b"NLXPOL01");
    p[8..10].copy_from_slice(&1u16.to_le_bytes());
    p[16..48].copy_from_slice(&[0x11; 32]);
    p[48..56].copy_from_slice(&0x21u64.to_le_bytes());
    p[56..88].copy_from_slice(&[0x22; 32]);
    p[88..120].copy_from_slice(&[0x33; 32]);
    p[120..128].copy_from_slice(&[0x44; 8]);
    for b in [0usize, 1, 231, 511, 512] {
        p[128 + b / 8] |= 1 << (b % 8);
    }
    p
}

#[test]
fn app_id_is_the_policy_crates_pinned_vector() {
    let p = c1_policy();
    let policy_hash = blake3::hash(&[&b"NONOS.linux.policy.v1"[..], &p].concat());
    assert_eq!(
        hex(policy_hash.as_bytes()),
        "aeb8c5679d33b3ba33e89e1b06c6603b212587470c898320af88c163b590c851"
    );
    assert_eq!(
        hex(&app_id(&p, &[0x11; 32])),
        "372ece38921fb4fe5f30f1609cd036526930a6cdf406b555620a9bb8aa4d2cfe"
    );
    let mut set = [0u8; 65];
    set.copy_from_slice(&p[128..193]);
    let on: Vec<usize> = (0..513).filter(|&b| allows(&set, b)).collect();
    assert_eq!(on, [0, 1, 231, 511, 512]);
}
