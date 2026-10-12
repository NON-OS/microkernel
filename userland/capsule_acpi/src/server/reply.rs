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

//! Building the replies protocol.rs describes.

use super::brightness;
use super::protocol::{EINVAL, ENOENT, OP_BRIGHTNESS, OP_DEVICE, OP_SET_BRIGHTNESS, OP_STATUS};
use crate::boot::{path, Acpi};
use alloc::vec::Vec;

fn status(acpi: &Acpi) -> Vec<u8> {
    let mut v = Vec::new();
    let routes: usize = acpi.routes.iter().map(|(_, r)| r.len()).sum();
    v.extend_from_slice(&(acpi.devices.len() as u32).to_le_bytes());
    v.extend_from_slice(&(routes as u32).to_le_bytes());
    v.extend_from_slice(&acpi.scis.to_le_bytes());
    v.extend_from_slice(&acpi.gpe_runs.to_le_bytes());
    v.extend_from_slice(&acpi.host.refused.to_le_bytes());
    let armed = acpi.events.as_ref().map_or(0, |e| e.armed.len());
    v.extend_from_slice(&(armed as u32).to_le_bytes());
    v.extend_from_slice(&acpi.aml.rt.dropped_notifications.to_le_bytes());
    v
}

fn device(acpi: &Acpi, index: usize) -> Option<Vec<u8>> {
    let d = acpi.devices.get(index)?;
    let mut v = Vec::new();
    v.extend_from_slice(&d.sta.to_le_bytes());
    v.extend_from_slice(&d.adr.unwrap_or(u64::MAX).to_le_bytes());
    v.extend_from_slice(&d.uid.unwrap_or(u64::MAX).to_le_bytes());
    let hid = d.hid.as_ref().map_or(&[][..], |h| h.bytes());
    let p = path(&acpi.aml, d.id);
    for s in [hid, p.as_bytes()] {
        let s = &s[..s.len().min(200)];
        v.push(s.len() as u8);
        v.extend_from_slice(s);
    }
    Some(v)
}

/// The reply to `req`: status word, then body.
pub fn answer(acpi: &mut Acpi, req: &[u8]) -> Vec<u8> {
    let index =
        |r: &[u8]| r.get(1..5).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize);
    let (code, body) = match req.first().copied() {
        Some(OP_STATUS) => (0, status(acpi)),
        Some(OP_DEVICE) => match index(req).and_then(|i| device(acpi, i)) {
            Some(b) => (0, b),
            None => (ENOENT, Vec::new()),
        },
        Some(OP_BRIGHTNESS) => brightness::get(acpi),
        Some(OP_SET_BRIGHTNESS) => brightness::set(acpi, req),
        _ => (EINVAL, Vec::new()),
    };
    let mut out = code.to_le_bytes().to_vec();
    out.extend_from_slice(&body);
    out
}
