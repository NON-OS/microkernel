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

//! The amnesic persist rule, every case: a boot that keeps nothing lets
//! only zeros through, and only as a withdrawal.

#[path = "../../capsule_vfs/src/server/handlers/persist_rule.rs"]
mod rule;

use rule::{keep, Keep};

#[test]
fn a_keeping_boot_writes_whatever_it_is_given() {
    for data in [&b""[..], &[0u8; 32][..], b"wallet vault", &[0xff; 4096][..]] {
        assert_eq!(keep(true, data), Some(Keep::Record));
    }
}

#[test]
fn an_amnesic_boot_refuses_any_byte_that_is_not_zero() {
    for at in [0usize, 1, 31, 4095] {
        let mut data = vec![0u8; 4096];
        data[at] = 1;
        assert_eq!(keep(false, &data), None, "a one at {at}");
    }
    assert_eq!(keep(false, b"name-of-a-secret"), None);
}

#[test]
fn an_amnesic_boot_takes_zeros_and_an_empty_payload_only_as_a_withdrawal() {
    for len in [0usize, 1, 32, 4096] {
        assert_eq!(keep(false, &vec![0u8; len]), Some(Keep::Withdraw), "{len} zeros");
    }
}
