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

//! The rollback floor by profile (REVIEW R8): a readable TPM counter is the
//! floor everywhere; without one, Hardened and Air-Gapped refuse to boot and
//! every other profile boots with the protection shown off.

use crate::menu::types::SecurityMode::{self, *};
use crate::security::anti_rollback::{floor_rule, Floor};

const EVERY_MODE: [SecurityMode; 6] =
    [Development, Standard, Hardened, SafeMode, NetworkIsolated, Recovery];

#[test]
fn a_readable_counter_is_the_floor_in_every_profile() {
    for mode in EVERY_MODE {
        for f in [0, 1, 7, u64::MAX] {
            assert_eq!(floor_rule(Some(f), mode.requires_tpm()), Floor::Held(f), "{mode:?}");
        }
    }
}

#[test]
fn without_a_counter_hardened_and_air_gapped_refuse() {
    for mode in EVERY_MODE {
        let want = match mode {
            Hardened | NetworkIsolated => Floor::Refuse,
            _ => Floor::Unprotected,
        };
        assert_eq!(floor_rule(None, mode.requires_tpm()), want, "{mode:?}");
    }
}
