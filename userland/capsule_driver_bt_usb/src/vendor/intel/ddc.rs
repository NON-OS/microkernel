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

//! The device configuration (DDC) a booted controller takes: entries of a
//! length octet and that many more (a two-octet id, then its value), each
//! sent whole, length first, as one Intel Write DDC (0xfc8b)
//! (`btintel_load_ddc_config`). The controller works without them, so a
//! refusal stops the rest but not the bring-up.

use alloc::vec::Vec;

use super::wire::Wire;
use nonos_bt_hci::usb::UsbPipes;
use nonos_bt_hci::HciError;

const WRITE_DDC: u16 = 0xfc8b;

/// The entries in order; `None` if the last runs past the end.
pub fn entries(mut ddc: &[u8]) -> Option<Vec<&[u8]>> {
    let mut out = Vec::new();
    while let [len, ..] = ddc {
        let (entry, rest) = ddc.split_at_checked(usize::from(*len) + 1)?;
        out.push(entry);
        ddc = rest;
    }
    Some(out)
}

pub fn load<P: UsbPipes>(w: &mut Wire<P>, ddc: &[u8]) -> Result<(), HciError> {
    let Some(all) = entries(ddc) else {
        (w.say)(&[b"intel: DDC file malformed, none sent"]);
        return Ok(());
    };
    for entry in all {
        match w.command(WRITE_DDC, entry) {
            Ok(_) => {}
            Err(HciError::Status(..)) => {
                (w.say)(&[b"intel: a DDC entry was refused, the rest not sent"]);
                return Ok(());
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}
