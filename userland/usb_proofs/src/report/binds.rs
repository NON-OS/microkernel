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

//! Configurations bound as the driver binds them: every HID interface the
//! walk offers, each one in report protocol kept only if `device_for` reads
//! a report descriptor it can drive, here the one the device is made to
//! send.

use crate::config_blob::{config, HID, INTERRUPT};
use crate::descriptors::{hid_bindings, HidBinding, HidKind};
use crate::orchestrator::device_for::device_for;
use crate::xhci::report_desc;

/// The interfaces bound, each with the kind it is read as, when each one in
/// report protocol sends `desc`, a descriptor whose length its HID
/// descriptor gives.
pub fn bound(cfg: &[u8], desc: &[u8]) -> Vec<(HidKind, u8)> {
    report_desc::serve(desc);
    let report_len = desc.len() as u16;
    let all = hid_bindings(cfg).expect("well formed");
    let kept = all.into_iter().filter_map(|b| device_for(7, 1, HidBinding { report_len, ..b }));
    kept.map(|(b, _)| (b.kind, b.interface_number)).collect()
}

/// One HID interface in report protocol, with packets of `mps` bytes.
pub fn one_interface(mps: u16) -> Vec<u8> {
    config().iface(0, HID, 0, 0).hid().ep(0x81, INTERRUPT, mps, 10).build()
}
