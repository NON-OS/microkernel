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

//! A primary given back: its share released first, then its memory
//! unmapped, so the compositor's attachment never outlives the pages it
//! reads. The share is the kernel's; the pages are only this capsule's.

use nonos_devmodel::FakeBar;
use nonos_gfx_serve::Platform;
use nonos_libc::{given_back, hold, lend, registered, SHARED_HANDLE_BASE};

use super::pool::Pool;

#[test]
fn a_primarys_share_is_released_before_its_memory_is_unmapped() {
    let memory = FakeBar::new(16384);
    lend(memory.base(), 16384);
    // The share names the compositor, so one must be running (K20).
    hold(b"compositor", Some(42));
    let mut p = Pool::default();
    let r = p.alloc(64 * 48 * 4).expect("the primary's memory");
    let handle = p.register(r, 64, 48, 64 * 4).expect("registered and shared");
    assert_eq!(registered().last().map(|d| d.base_va), Some(memory.base()));
    let before = given_back().len();
    p.release(Some(handle), r);
    let back = &given_back()[before..];
    assert_eq!(back, [("surface_release", handle), ("munmap", memory.base())]);
    assert!(handle as i64 >= SHARED_HANDLE_BASE, "the wire is handed the share");
}
