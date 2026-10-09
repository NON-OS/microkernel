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

//! A wallpaper the catalog cannot serve says why on the serial line, once.
//! The wallpaper service asks again every few seconds; before, every refusal
//! was silent and the desktop stayed on its built in picture with nothing
//! saying which step broke.

use nonos_libc::take_said;

use crate::catalog::{get_slug, with_bytes, Fetch};
use crate::collection::{files, flip, installed, reset, TURN};

fn said() -> Vec<String> {
    take_said().into_iter().map(|l| String::from_utf8(l).unwrap()).collect()
}

#[test]
fn a_changed_byte_is_named_as_a_pin_mismatch_once() {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    reset();
    let _ = said();
    let at: usize = files()[..40].iter().map(|(_, b)| b.len()).sum::<usize>() + 7;
    flip(at);
    assert_eq!(with_bytes(40, |_| ()), Err(Fetch::Unavailable));
    assert_eq!(with_bytes(40, |_| ()), Err(Fetch::Unavailable));
    let slug = String::from_utf8(get_slug(40).unwrap().to_vec()).unwrap();
    let want = format!("[WALLPAPER-CATALOG] {slug}: its bytes are not its pinned SHA-256\n");
    assert_eq!(said(), vec![want]);
    reset();
}

#[test]
fn a_wallpaper_the_disk_does_not_carry_names_the_failed_step() {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    reset();
    let _ = said();
    installed(&[0]);
    assert_eq!(with_bytes(50, |_| ()), Err(Fetch::Unavailable));
    let slug = String::from_utf8(get_slug(50).unwrap().to_vec()).unwrap();
    let lines = said();
    assert_eq!(lines.len(), 1, "{lines:?}");
    assert!(lines[0].starts_with(&format!("[WALLPAPER-CATALOG] {slug}: ")), "{lines:?}");
    assert!(lines[0].ends_with('\n') && lines[0].len() > slug.len() + 24, "{lines:?}");
    reset();
}

#[test]
fn a_wallpaper_served_says_nothing() {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    reset();
    let _ = said();
    assert_eq!(with_bytes(30, |b| b.len()), Ok(files()[30].1.len()));
    assert!(said().is_empty());
}
