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

//! The kept status, and the outcome read from it.

use super::kept_table::{Kept, KEPT_SLOTS};
use super::outcome_rule::{read, Ended};

#[test]
fn a_kept_status_is_read_as_often_as_asked() {
    let mut k = Kept::new();
    assert!(k.watch(0x28));
    assert_eq!(k.status(0x28), None, "not ended yet");
    k.note(0x28, 3);
    assert_eq!((k.status(0x28), k.status(0x28)), (Some(3), Some(3)));
}

#[test]
fn only_an_asked_for_status_is_kept() {
    let mut k = Kept::new();
    k.note(0x28, 3);
    assert!(k.watch(0x28));
    assert_eq!(k.status(0x28), None, "an end before the ask is not kept");
    assert!(!k.watch(0));
    k.note(0, 3);
    assert_eq!(k.status(0), None);
    for pid in 1..KEPT_SLOTS as u32 {
        assert!(k.watch(0x100 + pid));
    }
    assert!(!k.watch(0x200), "every slot taken");
    assert!(k.watch(0x28), "asking again keeps its slot");
}

#[test]
fn a_status_consumed_before_init_reads_it_still_says_install() {
    assert_eq!(read(0x28, Some(3), None, None), (Ended::Installer, 0));
    assert_eq!(read(0x28, Some(3 | 5 << 8), None, None), (Ended::Installer, 5));
    assert_eq!(read(0x28, Some(0), None, None), (Ended::Desktop, 0));
}

#[test]
fn the_kept_status_is_read_first() {
    assert_eq!(read(0x28, Some(3), Some(0), Some(0)).0, Ended::Installer);
    assert_eq!(read(0x28, None, Some(3), Some(0)).0, Ended::Installer);
    assert_eq!(read(0x28, None, None, Some(3)).0, Ended::Installer);
}

#[test]
fn no_status_or_no_setup_is_unfinished() {
    assert_eq!(read(0x28, None, None, None), (Ended::Unfinished, 0));
    assert_eq!(read(0, Some(3), Some(3), Some(3)), (Ended::Unfinished, 0));
    assert_eq!(read(0x28, Some(-1), None, None), (Ended::Unfinished, 0));
    assert_eq!(read(0x28, Some(1 << 16), None, None), (Ended::Unfinished, 0));
}
