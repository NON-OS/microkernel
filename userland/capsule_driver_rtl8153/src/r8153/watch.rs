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

//! The link, read from PLA_PHYSTATUS at most once a second and kept. The
//! chip's link interrupt comes on endpoint 3, which driver.xhci0 does not
//! configure beside the bulk pair, so it is polled, as Linux polls it
//! when the interrupt does not come. A link that comes up starts traffic
//! (Linux set_carrier); one that goes down is only noted.

use nonos_libc::Deadline;
use nonos_usbnet::Bus;

use super::enable::enable;
use super::link::Rtl8153;
use super::ocp::{read_word, PLA};
use super::regs::bits::{FULL_DUP, LINK_STATUS, SPEED_10, SPEED_100, SPEED_1000};
use super::regs::pla::PHYSTATUS;

const LOOK_EVERY_MS: u64 = 1_000;

pub(super) fn watch<B: Bus>(nic: &mut Rtl8153<B>) -> Result<(), i32> {
    if !nic.next_look.expired() {
        return Ok(());
    }
    nic.next_look = Deadline::after_ms(LOOK_EVERY_MS);
    let speed = read_word(&mut nic.dev, PLA, PHYSTATUS)?;
    let up = speed & LINK_STATUS != 0;
    if up && !nic.link {
        if let Err((what, e)) = enable(&mut nic.dev, nic.version, speed) {
            (nic.note)(&[b"link up, traffic not started: ", what.as_bytes()]);
            return Err(e);
        }
        (nic.note)(&[b"link up, ", rate(speed), duplex(speed)]);
    } else if !up && nic.link {
        (nic.note)(&[b"link down"]);
    }
    nic.link = up;
    Ok(())
}

fn rate(speed: u16) -> &'static [u8] {
    match speed {
        s if s & SPEED_1000 != 0 => b"1000 Mb/s",
        s if s & SPEED_100 != 0 => b"100 Mb/s",
        s if s & SPEED_10 != 0 => b"10 Mb/s",
        _ => b"speed unknown",
    }
}

fn duplex(speed: u16) -> &'static [u8] {
    if speed & FULL_DUP != 0 {
        b" full duplex"
    } else {
        b" half duplex"
    }
}
