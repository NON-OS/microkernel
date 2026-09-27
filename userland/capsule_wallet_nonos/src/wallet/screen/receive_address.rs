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

//! The account's address as the 0x text a person reads, copies and scans.

use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::rect::Rect;
use crate::wallet::state::State;

pub fn address_hex(state: &State) -> String {
    let mut hex = String::from("0x");
    for b in state.address {
        hex.push_str(&alloc::format!("{b:02x}"));
    }
    hex
}

/// What is known about the address, as facts; returns their height.
pub fn facts(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let key = if state.vault_saved { "sealed to this machine" } else { "RAM only, gone at reboot" };
    let h = fact(fb, c.x, y, c.w, "network", "Ethereum mainnet");
    h + fact(fb, c.x, y + h, c.w, "key", key)
}
