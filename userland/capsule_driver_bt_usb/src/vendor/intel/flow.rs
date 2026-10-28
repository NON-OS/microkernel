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
//! asked again without it. A TLV part's firmware is sent and booted and
//! the part asked again, and once more if what runs is an intermediate
//! loader (Blazar, variant 0x1e, and later).

use super::legacy;
use super::name::stem;
use super::steps::{finish, load, running};
use super::tlv::{self, Version, IMG_OP};
use super::wire::Wire;
use nonos_bt_hci::usb::UsbPipes;
use nonos_bt_hci::HciError;

const READ_VERSION: u16 = 0xfc05;

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
        _ => Err(w.refuse(&[b"hardware variant not supported"])),
    }
}

fn fresh<P: UsbPipes>(w: &mut Wire<P>, mut v: Version) -> Result<(), HciError> {
    if v.img_type == IMG_OP {
        return running(w, &stem(&v, true), v.variant(), v.build);
    }
    for _ in 0..2 {
        load(w, &stem(&v, true), v.variant(), v.sbe_type)?;
        let ret = w.command(READ_VERSION, &[0xff])?;
        v = version(w, &ret)?;
        if v.img_type == IMG_OP {
            return finish(w, &stem(&v, false));
        }
    }
    Err(w.refuse(&[b"firmware booted but is not operational"]))
}

fn version<P: UsbPipes>(w: &Wire<P>, ret: &[u8]) -> Result<Version, HciError> {
    match tlv::parse(ret) {
        Some(v) if v.platform() == 0x37 => Ok(v),
        Some(_) => Err(w.refuse(&[b"not an Intel Bluetooth platform"])),
        None => Err(w.refuse(&[b"version malformed"])),
    }
}
