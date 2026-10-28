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

//! The link, read from the chip's link word on the frame service's clock
//! (Nic::tick) one request in LOOK_EVERY, as Linux v6.12 reads the same
//! word on the chip's link interrupt. Up turns receive and transmit on
//! (link_up.rs) and only then reads up; down reads down at once, then
//! gates receive and resets the MAC (link_down.rs).

use nonos_usbnet::Bus;

use super::link_down::LINK_DOWN;
use super::link_up::link_up;
use super::link_word::{is_up, WORD};
use super::ocp::read;
use super::pause::Pause;
use super::run::run;
use super::state::Rtl;

/// Each look is one control transfer through driver.xhci0; net.core asks
/// for the link once a second.
pub const LOOK_EVERY: u32 = 16;

pub(super) fn watch<B: Bus + Pause>(r: &mut Rtl<B>) -> Result<(), i32> {
    r.looks = r.looks.wrapping_add(1);
    if !r.looks.is_multiple_of(LOOK_EVERY) {
        return Ok(());
    }
    look(r)
}

/// Read the link word and follow it.
pub fn look<B: Bus + Pause>(r: &mut Rtl<B>) -> Result<(), i32> {
    let word = read(&mut r.bus, WORD)?;
    match (is_up(word), r.link) {
        (true, false) => {
            let steps = link_up(r.model, r.speed, word);
            run(&mut r.bus, &mut r.page, &steps).map_err(|s| s.errno())?;
            r.link = true;
        }
        (false, true) => {
            r.link = false;
            run(&mut r.bus, &mut r.page, &LINK_DOWN).map_err(|s| s.errno())?;
        }
        _ => {}
    }
    Ok(())
}
