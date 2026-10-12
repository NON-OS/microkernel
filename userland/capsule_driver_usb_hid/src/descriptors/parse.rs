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

use alloc::vec::Vec;

use crate::protocol::MAX_HID_BINDINGS;

use super::binding::HidBinding;
use super::records::{interface, maybe_push, with_report_len};
use super::types::{DT_CONFIGURATION, DT_ENDPOINT, DT_HID, DT_INTERFACE};

/// Every HID interrupt IN endpoint in configuration `raw`, up to the cap. A
/// boot keyboard or mouse is read in its boot layout; any other HID
/// interface, a keyboard's media keys among them, is bound later only if its
/// report descriptor names reports this driver reads.
pub fn hid_bindings(raw: &[u8]) -> Result<Vec<HidBinding>, ()> {
    validate_config(raw)?;
    let total = u16::from_le_bytes([raw[2], raw[3]]) as usize;
    let mut out = Vec::new();
    let mut iface = None;
    let mut i = 9usize;
    while i + 2 <= total {
        let len = raw[i] as usize;
        if len < 2 || i + len > total {
            return Err(());
        }
        let rec = &raw[i..i + len];
        match rec[1] {
            // A cut interface record still opens a new interface: the
            // endpoints after it are its own, never the one before it.
            DT_INTERFACE if len >= 9 => iface = Some(interface(rec)),
            DT_INTERFACE => iface = None,
            DT_HID => iface = iface.map(|f| with_report_len(f, rec)),
            DT_ENDPOINT if len >= 7 => maybe_push(&mut out, iface, rec),
            _ => {}
        }
        if out.len() == MAX_HID_BINDINGS {
            break;
        }
        i += len;
    }
    Ok(out)
}

fn validate_config(raw: &[u8]) -> Result<(), ()> {
    if raw.len() < 9 || raw[0] < 9 || raw[1] != DT_CONFIGURATION {
        return Err(());
    }
    let total = u16::from_le_bytes([raw[2], raw[3]]) as usize;
    if total < 9 || total > raw.len() {
        return Err(());
    }
    Ok(())
}
