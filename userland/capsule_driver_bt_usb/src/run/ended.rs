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

//! After a link: one the peer ended needs nothing more; one this host gave
//! up on is ended with Disconnect, and the next look waits, so a device
//! that will not pair is not asked again and again.

use nonos_bt_hci::usb::UsbHci;
use nonos_bt_hci::{HciError, Host};

use super::desk::Desk;
use super::say::say;
use crate::link::{hang_up, End, FORGOTTEN};
use crate::nbtc::ops::IDLE;
use crate::usb::XhciPipes;

/// After a link this host gave up on, before the next look.
const BACK_OFF_MS: u64 = 30_000;

pub fn ended(
    host: &mut Host<UsbHci<XhciPipes>>,
    end: Result<End, HciError>,
    handle: u16,
    desk: &Desk,
) -> Result<(), HciError> {
    desk.board().ended();
    match end? {
        End::Lost(_) => say(&[b"link lost"]),
        End::GaveUp(why) => {
            say(&[b"link ended: ", why.as_bytes()]);
            hang_up(host, handle)?;
            // The user ended this one; nothing about the device was wrong.
            if why != FORGOTTEN {
                desk.board().state = IDLE;
                desk.wait(BACK_OFF_MS);
            }
        }
    }
    Ok(())
}
