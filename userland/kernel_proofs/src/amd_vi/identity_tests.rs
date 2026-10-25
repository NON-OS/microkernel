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

//! The posture line's identity count on AMD-Vi is a running sum of
//! `identity_change` over every device table write. These hold that sum to
//! the table itself.

use super::dte::{blocked, passthrough, translated, Dte};
use super::identity_change::{identity_change, is_passthrough};

const ZERO: Dte = [0; 4];

#[test]
fn each_kind_of_write_moves_the_count_by_what_it_does() {
    let mapped = translated(0x1234_5000, 4, 7);
    assert_eq!(identity_change(ZERO, passthrough()), 1);
    assert_eq!(identity_change(blocked(), passthrough()), 1);
    assert_eq!(identity_change(passthrough(), mapped), -1);
    assert_eq!(identity_change(passthrough(), blocked()), -1);
    assert_eq!(identity_change(passthrough(), passthrough()), 0);
    assert_eq!(identity_change(blocked(), mapped), 0);
    assert_eq!(identity_change(mapped, blocked()), 0);
}

#[test]
fn only_the_passthrough_entry_counts() {
    assert!(is_passthrough(passthrough()));
    for entry in [ZERO, blocked(), translated(0x1000, 4, 1), translated(0x1000, 6, 9)] {
        assert!(!is_passthrough(entry));
    }
    // Mode 0 walks no table whatever the domain, and a writer entry with no
    // translation information is what the stray scan refuses: both untranslated.
    assert!(is_passthrough(translated(0, 0, 0)));
    assert!(is_passthrough(translated(0x1000, 0, 5)));
    assert!(is_passthrough([1 | (1 << 62), 0, 0, 0]));
}

/// Bring-up, claims, releases and repeated names in any order: the sum of
/// the changes always equals the passthrough entries in the table.
#[test]
fn the_running_sum_is_the_table() {
    let mut table = [ZERO; 64];
    let mut sum: i64 = 0;
    let mut seed: u64 = 0x2545_F491_4F6C_DD1D;
    for step in 0..20_000u32 {
        seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        let id = (seed >> 33) as usize % table.len();
        let after = match (seed >> 40) % 4 {
            0 => passthrough(),
            1 => blocked(),
            2 => translated(((seed >> 12) & 0xFFFF_F000) | 0x1000, 4, (step % 9) as u16 + 1),
            _ => ZERO,
        };
        sum += i64::from(identity_change(table[id], after));
        table[id] = after;
        let count = table.iter().filter(|e| is_passthrough(**e)).count() as i64;
        assert_eq!(sum, count, "step {step}");
    }
}
