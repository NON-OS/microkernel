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

//! The steps both version forms share: a controller already running
//! firmware, one taking it, and the DDC once it runs.

use super::boot::{boot, to_bootloader};
use super::carried::carried;
use super::cmds::boot_params;
use super::ddc;
use super::download::download;
use super::end::{end, say};
use super::sfi::layout;
use super::wire::Wire;
use nonos_bt_hci::usb::UsbPipes;
use nonos_bt_hci::HciError;

/// Nothing is sent when what runs is the carried build, or none is carried
/// for it; other firmware is sent back to its bootloader to be found again.
pub fn running<P: UsbPipes>(
    w: &mut Wire<P>,
    stem: &str,
    v: u8,
    build: [u8; 3],
) -> Result<(), HciError> {
    let Some(c) = carried(stem) else {
        say(w.say, b"running firmware kept, none carried: ", stem);
        return end(w);
    };
    let ours = layout(c.sfi, v, 0).ok().and_then(|l| boot_params(&c.sfi[l.payload..]));
    if ours.is_some_and(|(_, b)| b == build) {
        say(w.say, b"firmware already running: ", stem);
        return end(w);
    }
    to_bootloader(w);
    Err(w.refuse(&[b"running firmware is not ", stem.as_bytes(), b", sent back to load it"]))
}

/// The carried firmware `stem` names, sent and booted.
pub fn load<P: UsbPipes>(w: &mut Wire<P>, stem: &str, v: u8, sbe: u8) -> Result<(), HciError> {
    let Some(c) = carried(stem) else {
        return Err(w.refuse(&[b"firmware not carried: ", stem.as_bytes(), b".sfi"]));
    };
    let l = layout(c.sfi, v, sbe).map_err(|why| w.refuse(&[why.as_bytes()]))?;
    say(w.say, b"sending ", stem);
    let addr = download(w, c.sfi, &l)?;
    boot(w, addr)
}

pub fn finish<P: UsbPipes>(w: &mut Wire<P>, stem: &str) -> Result<(), HciError> {
    say(w.say, b"booted, DDC from ", stem);
    carried(stem).map_or(Ok(()), |c| ddc::load(w, c.ddc))?;
    end(w)
}
