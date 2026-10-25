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

//! The release record is read as untrusted: exactly the shape the seal
//! writes is accepted, and every other shape is refused with a reason.

use crate::release::record::{parse, Item};
use serde_json::{json, Value};

pub const H: &str = "a3f1c9b2d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a4b5c6d7e8f";

pub fn rec(config: Value, sealed: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({"format": "nonos-release/1", "config": config, "sealed": sealed}))
        .unwrap()
}
fn one(sealed: Value) -> Result<crate::release::record::Record, String> {
    parse(&rec(json!({"release": false}), sealed))
}

#[test]
fn the_shape_the_seal_writes_is_read_whole() {
    let r = one(
        json!({"usb": {"file": "nonos.img", "sha256": H}, "esp": {"dir": "esp", "files": {"EFI/x": H}}}),
    );
    let r = r.unwrap();
    assert_eq!((r.release, r.sealed.len()), (false, 2));
    assert!(r.sealed.iter().any(|s| s.item == Item::File(H.into()) && s.rel == "nonos.img"));
    assert!(r.sealed.iter().any(|s| s.item == Item::Dir(vec![("EFI/x".into(), H.into())])));
}

#[test]
fn a_path_that_could_leave_the_release_is_refused() {
    for p in ["../x", "/etc/passwd", "a/../../b", "a//b", "./a", "a/.", "a\\b", "", "a\0b"] {
        assert!(one(json!({"usb": {"file": p, "sha256": H}})).is_err(), "file {p:?}");
        assert!(one(json!({"esp": {"dir": "esp", "files": {p: H}}})).is_err(), "in dir {p:?}");
        assert!(one(json!({"esp": {"dir": p, "files": {}}})).is_err(), "dir {p:?}");
    }
}

#[test]
fn a_digest_not_in_the_form_the_seal_writes_is_refused() {
    let upper = H.to_uppercase();
    for h in ["", &H[..63], &format!("{H}0"), &"g".repeat(64), &upper] {
        assert!(one(json!({"usb": {"file": "a", "sha256": h}})).is_err(), "{h:?}");
    }
}
