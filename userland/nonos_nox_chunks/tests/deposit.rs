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

#[path = "common/k.rs"]
mod k;

use k::{kec, K};
use nonos_nox_chunks::{clone_initcode_hash, create2, deposit_address};

fn hex20(s: &str) -> [u8; 20] {
    let mut out = [0u8; 20];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap();
    }
    out
}

#[test]
fn create2_meets_eip_1014s_examples() {
    let zero = [0u8; 32];
    assert_eq!(
        create2(&K, &[0; 20], &zero, &kec(&[&[0x00]])),
        hex20("4d1a2e2bb4f88f0250f26ffff098b0b30b26bf38")
    );
    let deployer = hex20("deadbeef00000000000000000000000000000000");
    assert_eq!(
        create2(&K, &deployer, &zero, &kec(&[&[0x00]])),
        hex20("b928f69bb1d91cd65274e3c79d8986362984fda3")
    );
    let mut salt = [0u8; 32];
    salt[12..].copy_from_slice(&hex20("feed000000000000000000000000000000000000"));
    assert_eq!(
        create2(&K, &deployer, &salt, &kec(&[&[0x00]])),
        hex20("d04116cdd17bebe565eb2422f2497e06cc1c9833")
    );
    let code = [0xde, 0xad, 0xbe, 0xef];
    assert_eq!(
        create2(&K, &[0; 20], &zero, &kec(&[&code])),
        hex20("70f2b2914a2a4b783faefb75f459a580616fcb5e")
    );
}

#[test]
fn a_deposit_address_is_a_clone_per_order() {
    let implementation = [0xAB; 20];
    let mut code = hex20("3d602d80600a3d3981f3363d3d373d3d3d363d73").to_vec();
    code.extend_from_slice(&implementation);
    code.extend_from_slice(&[
        0x5a, 0xf4, 0x3d, 0x82, 0x80, 0x3e, 0x90, 0x3d, 0x91, 0x60, 0x2b, 0x57, 0xfd, 0x5b, 0xf3,
    ]);
    assert_eq!(code.len(), 55);
    let h = clone_initcode_hash(&K, &implementation);
    assert_eq!(h, kec(&[&code]));
    let escrow = [0xE5; 20];
    let a = deposit_address(&K, &escrow, &[1; 32], &h);
    assert_eq!(a, create2(&K, &escrow, &[1; 32], &h));
    assert_ne!(a, deposit_address(&K, &escrow, &[2; 32], &h), "one per order");
}
