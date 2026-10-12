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

//! The control-method buttons and the lid: found by _HID or _CID, and
//! _LID read.

use crate::button::{button_of, lid_open, Button};
use crate::enc::*;
use crate::platform::{DeviceInfo, HardwareId};
use crate::run::eval;

fn dev(hid: Option<&[u8; 7]>, cids: &[&[u8; 7]]) -> DeviceInfo {
    let eisa = |id: &[u8; 7]| HardwareId::Eisa(*id);
    let cids = cids.iter().map(|c| eisa(c)).collect();
    DeviceInfo { id: 1, hid: hid.map(eisa), cids, uid: None, adr: None, sta: 0xF }
}

#[test]
fn buttons_by_hid_or_cid() {
    assert_eq!(button_of(&dev(Some(b"PNP0C0C"), &[])), Some(Button::Power));
    assert_eq!(button_of(&dev(Some(b"PNP0C0E"), &[])), Some(Button::Sleep));
    assert_eq!(button_of(&dev(None, &[b"PNP0C0D"])), Some(Button::Lid));
    assert_eq!(button_of(&dev(Some(b"PNP0C0A"), &[])), None);
}

#[test]
fn lid_reads() {
    assert_eq!(lid_open(&eval(&ret(&int(0)), vec![]).unwrap()), Some(false));
    assert_eq!(lid_open(&eval(&ret(&int(1)), vec![]).unwrap()), Some(true));
    assert_eq!(lid_open(&eval(&ret(&string("x")), vec![]).unwrap()), None);
}
