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
//! Subscribers hear that focus moved whenever it did, by whatever path:
//! The notice the WM sends is the layout abi/wire.toml pins as
//! [wm.notify_v1]: each field at its offset, and each kind its number.

use crate::protocol::{
    encode_notify, NOTIFY_KIND_CLOSED, NOTIFY_KIND_FOCUSED, NOTIFY_KIND_FULL_SCREEN,
    NOTIFY_KIND_KEYS, NOTIFY_KIND_OPENED, NOTIFY_LEN,
};

const WIRE: &str = include_str!("../../../abi/wire.toml");

/// The value of `key` in table `table`: a number, or the offset of a field.
fn pinned(table: &str, key: &str) -> u32 {
    let body = WIRE.split(&format!("[{table}]\n")).nth(1).expect(table);
    let body = body.split("\n[").next().unwrap();
    let line = body.lines().find(|l| l.split('=').next().unwrap().trim() == key).expect(key);
    let v = line.split_once('=').unwrap().1;
    let v = v.split("offset =").nth(1).unwrap_or(v);
    v.trim().trim_start_matches('{').split([',', '}']).next().unwrap().trim().parse().expect(key)
}

#[test]
fn the_notice_is_the_pinned_layout() {
    assert_eq!(pinned("wm.notify_v1", "size_bytes") as usize, NOTIFY_LEN);
    let mut frame = [0u8; NOTIFY_LEN];
    encode_notify(&mut frame, 0x1111_1111, 0x2222_2222, 0x3333_3333, 0x4444_4444, 0x5555_5555);
    let word =
        |at: u32| u32::from_le_bytes(frame[at as usize..at as usize + 4].try_into().unwrap());
    assert_eq!(word(pinned("wm.notify_v1", "magic")), u32::from_le_bytes(*b"VMWN"));
    let half = |at: u32| u16::from_le_bytes([frame[at as usize], frame[at as usize + 1]]);
    assert_eq!(
        (half(pinned("wm.notify_v1", "version")), half(pinned("wm.notify_v1", "flags"))),
        (1, 0)
    );
    for (field, v) in [
        ("event_kind", 0x1111_1111),
        ("owner_pid", 0x2222_2222),
        ("window_id", 0x3333_3333),
        ("x", 0x4444_4444),
        ("y", 0x5555_5555),
    ] {
        assert_eq!(word(pinned("wm.notify_v1", field)), v, "{field}");
    }
}

#[test]
fn each_kind_is_its_pinned_number() {
    let kinds = [
        ("opened", NOTIFY_KIND_OPENED),
        ("closed", NOTIFY_KIND_CLOSED),
        ("full_screen", NOTIFY_KIND_FULL_SCREEN),
        ("focused", NOTIFY_KIND_FOCUSED),
        ("keys", NOTIFY_KIND_KEYS),
    ];
    for (name, kind) in kinds {
        assert_eq!(pinned("wm.notify_v1.kinds", name), kind, "{name}");
    }
}
