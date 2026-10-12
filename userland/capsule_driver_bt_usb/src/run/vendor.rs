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

//! The vendor's setup, for a controller that needs one before bring-up:
//! Realtek's, with its files from the store. A controller that needs none
//! goes on as before; one whose setup fails is given back unpatched.

use nonos_bt_hci::usb::UsbHci;
use nonos_bt_hci::{HciError, Host};

use super::say::say;
use super::store::Store;
use crate::rtl::{setup, Done, Refused, CHIPS};
use crate::usb::XhciPipes;

pub fn vendor(host: &mut Host<UsbHci<XhciPipes>>) -> Result<(), HciError> {
    let why = match setup(host, &mut Store, CHIPS) {
        Ok(Done::NotOurs) => return Ok(()),
        Ok(Done::Patched(chip)) => {
            let fw = chip.firmware.file.as_bytes();
            say(&[b"Realtek ", chip.name.as_bytes(), b" patched with ", fw]);
            return Ok(());
        }
        Err(why) => why,
    };
    let (file, what) = why.words();
    say(&[b"Realtek setup: ", file.as_bytes(), what]);
    Err(match why {
        Refused::Hci(e) => e,
        _ => HciError::Vendor,
    })
}
