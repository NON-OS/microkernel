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

//! What a bring-up says of the firmware that came alive, the station address
//! it draws, and whether the firmware runs this driver's join layout.

use crate::firmware::gen3::alive::Alive;
use crate::firmware::gen3::station::ids::check as check_join_api;
use crate::firmware::gen3::ucode::Ucode;

use super::super::address::draw;
use super::super::say::Line;

pub(super) fn say_alive(alive: &Alive, has_pnvm: bool, given: Option<u32>) {
    Line::new()
        .text(b"firmware alive: status ")
        .hex(u32::from(alive.status))
        .text(b" umac ")
        .num(alive.umac_major)
        .text(b".")
        .num(alive.umac_minor)
        .text(b" sku ")
        .hex(alive.sku_id[0])
        .send();
    match (has_pnvm, given) {
        (_, Some(v)) => Line::new().text(b"platform NVM section ").hex(v).text(b" given").send(),
        (true, None) if alive.sku_id != [0; 3] => {
            Line::new().text(b"no platform NVM section for this SKU: firmware defaults").send()
        }
        _ => {}
    }
}

/// The station address for this boot; without randomness the interface
/// takes the fixed one, which only a passive scan (sending nothing) uses.
pub(super) fn station_addr() -> Option<[u8; 6]> {
    let addr = draw();
    if addr.is_none() {
        Line::new().text(b"no randomness for a station address: scanning only").send();
    }
    addr
}

pub(super) fn join_layout(ucode: &Ucode<'_>) -> bool {
    match check_join_api(ucode) {
        Ok(()) => true,
        Err((group, cmd)) => {
            Line::new()
                .text(b"joining off: the firmware runs another layout of command ")
                .hex(u32::from(group))
                .text(b"/")
                .hex(u32::from(cmd))
                .send();
            false
        }
    }
}
