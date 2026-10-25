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

//! The load path's manifest check says what the verified spawn's own says:
//! a request `asked.rs` lets through is one `grant_within_manifest` admits,
//! and what a load installs never leaves a ceiling the manifest is within.

use crate::kernel_core::process_spawn::capsule_spawn::from_vfs::load::asked::beyond;
use crate::kernel_core::process_spawn::capsule_spawn::from_vfs::load::bound::installed;
use crate::security::capsule_manifest::CapsuleManifest;
use crate::spawn_caps::{grant_within_manifest, within_ceiling};

#[test]
fn asked_agrees_with_the_verified_spawn_over_many_masks() {
    let mut x: u64 = 0x2545_F491_4F6C_DD1D;
    for _ in 0..20_000 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        let (req, opt) = (x & 0x0F0F_0F0F, (x >> 4) & 0x00F0_00F0);
        let (asked, ceiling) = (x.rotate_left(17), x.rotate_left(41) | req | opt);
        let m = CapsuleManifest { required_caps: req, optional_caps: opt };
        assert_eq!(beyond(&m, asked) == 0, grant_within_manifest(req, opt, asked));
        assert!(within_ceiling(req, opt, ceiling));
        assert_eq!(installed(&m, asked) & !ceiling, 0, "a load stays under the ceiling");
    }
}
