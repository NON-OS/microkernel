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

//! `btintel_setup_combined`'s order. Read Version with parameter 0xff is
//! answered in TLV by TyP and later and in the old form by older parts;
//! JfP to CcP may answer in TLV yet are named by the old form, so they are
//! asked again without it. TLV parts are `fresh`'s.

use super::fresh::{fresh, version, READ_VERSION};
use super::legacy;
use super::wire::Wire;
use nonos_bt_hci::usb::UsbPipes;
use nonos_bt_hci::HciError;

pub fn run<P: UsbPipes>(w: &mut Wire<P>) -> Result<(), HciError> {
    let ret = w.command(READ_VERSION, &[0xff])?;
    if let Some(l) = legacy::parse(&ret) {
        return legacy::run(w, l);
    }
    let v = version(w, &ret)?;
    match v.variant() {
        0x11..=0x14 => match legacy::parse(&w.command(READ_VERSION, &[])?) {
            Some(l) => legacy::run(w, l),
            None => Err(w.refuse(&[b"version malformed"])),
        },
        0x17..=0x19 | 0x1b..=0x1e => fresh(w, v),
        _ => Err(w.refuse(&[b"hardware variant unknown to this loader"])),
    }
}
