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

//! F-102, the capsule's half: one path and reason per region space, only
//! memory, ports, config and the EC reached, EC space 256 single bytes, the
//! dispatch asking the table, and each refused region told once (64 at most).

#[path = "../../capsule_acpi/src/host/spaces.rs"]
mod spaces;
#[path = "../../capsule_acpi/src/host/told.rs"]
mod told;

use spaces::{ec_ok, path, Path};
use told::{tell, Tell, BOUND};

const REGION: &str = include_str!("../../capsule_acpi/src/host/region.rs");

#[test]
fn only_four_spaces_are_reached_and_each_says_why() {
    for space in 0..=u8::MAX {
        let (p, why) = path(space);
        assert!(!why.is_empty(), "{space}");
        let want = match space {
            0 => Path::Window,
            1 | 2 => Path::Kernel,
            3 => Path::Ec,
            _ => Path::Refused,
        };
        assert_eq!(p, want, "space {space:#x}");
    }
    assert!(
        REGION.matches("match path(a.space.0).0").count() == 2,
        "both directions ask the table"
    );
}

#[test]
fn ec_space_is_256_single_bytes() {
    assert!((0..=0xFF).all(|a| ec_ok(a, 1)));
    assert!(!ec_ok(0x100, 1) && !ec_ok(u64::MAX, 1));
    assert!([0, 2, 4, 8].iter().all(|w| !ec_ok(0x10, *w)));
}

#[test]
fn each_refused_region_is_told_once_and_the_telling_stops() {
    let mut told = vec![];
    assert_eq!([tell(&mut told, (9, 0x10)), tell(&mut told, (9, 0x10))], [Tell::New, Tell::Quiet]);
    let (mut r, mut said) = (0x2545_F491u64, 1);
    for _ in 0..20_000 {
        r ^= r << 13;
        r ^= r >> 7;
        r ^= r << 17;
        let key = ((r % 13) as u8, r % 97);
        if tell(&mut told, key) != Tell::Quiet {
            said += 1;
        }
        assert!(told.len() <= BOUND + 1);
    }
    assert_eq!(said, told.len(), "every told key remembered, none told twice");
    assert_eq!(said, BOUND + 1, "the bound reached, then one last line");
    assert_eq!(tell(&mut told, (200, 1)), Tell::Quiet, "past the bound, even a new one");
}
