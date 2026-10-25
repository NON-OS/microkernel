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

//! When keeping turns on, the RAM volume held for an installed disk is
//! dropped and nothing of it is written anywhere.

use crate::volume_glue::{before, flat, SWITCH};

const KEEP: &str = include_str!("../../../src/security/zerostate/keep.rs");

#[test]
fn keeping_on_drops_the_held_volume_and_copies_nothing() {
    assert!(flat(KEEP).contains("ifgranted&&!was{crate::fs::blockfs_volume::keeping_on();}"));
    let s = flat(SWITCH);
    let body = &s[s.find("fndrop_held()").expect("drop_held")..];
    before(&body, "if!drop_at_arm(HELD.swap(false,", "volume.take()");
    before(&body, "flush_held()", "ram::stop()");
    for write in ["write_or_create", "stream_write", "import(", "write(&"] {
        assert!(!body.contains(write), "the dropped volume is not copied (`{write}`)");
    }
}
