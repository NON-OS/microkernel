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

//! NCMP 0x0009 DISPLAY_HINT: its body, and the rule that only a service in
//! the kernel's HINT_SOURCES may send one.

use crate::display::hint_rule::{allowed, Hint, SOURCE_VMMDEV, SOURCE_VMWARE};
use crate::displays::HINT_SOURCES;

fn body(output: u32, w: u32, h: u32, source: u32) -> Vec<u8> {
    [output, w, h, source].iter().flat_map(|v| v.to_le_bytes()).collect()
}

#[test]
fn a_hint_is_sixteen_bytes_from_a_known_source() {
    let h = Hint::decode(&body(0, 1920, 1080, SOURCE_VMWARE)).unwrap();
    assert_eq!((h.output, h.width, h.height, h.source), (0, 1920, 1080, SOURCE_VMWARE));
    assert!(Hint::decode(&body(0, 1920, 1080, SOURCE_VMMDEV)).is_some());
    assert_eq!(Hint::decode(&body(0, 1920, 1080, 3)), None, "an unknown source");
    assert_eq!(Hint::decode(&body(0, 1920, 1080, 1)[..15]), None, "short");
    let mut long = body(0, 1920, 1080, 1);
    long.push(0);
    assert_eq!(Hint::decode(&long), None, "long");
}

#[test]
fn zero_by_zero_is_a_probe() {
    assert!(Hint::decode(&body(0, 0, 0, SOURCE_VMMDEV)).unwrap().is_probe());
    assert!(!Hint::decode(&body(0, 0, 768, SOURCE_VMMDEV)).unwrap().is_probe());
}

#[test]
fn only_a_named_source_is_heard_and_today_none_is() {
    /*
     * Until the owner accepts display hints (Decision 15) the list is
     * empty, so every sender is refused, whatever it claims to be.
     */
    assert!(HINT_SOURCES.is_empty());
    assert!(!allowed(7, Vec::new()));
    assert!(allowed(7, vec![3, 7]));
    assert!(!allowed(8, vec![3, 7]), "a sender whose pid no source holds");
    assert!(!allowed(0, vec![0]), "pid 0 is no process");
}
