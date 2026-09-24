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
use crate::protocol::{E_SHORT, STATUS_OK};

use super::u32_le::u32_le;
use crate::theme::store::{replace, Theme};

pub fn apply(payload: &[u8]) -> u16 {
    if payload.len() < 20 {
        return E_SHORT;
    }
    /*
     * Through the constructor, so a theme pushed over IPC gets the same derived
     * secondary text as one chosen in the settings panel. The wire carries the five
     * roles only: the derived pair is not a decision a sender gets to make, because
     * a sender could send one that fails the contrast floor.
     */
    replace(Theme::from_roles(
        u32_le(&payload[0..4]),
        u32_le(&payload[4..8]),
        u32_le(&payload[8..12]),
        u32_le(&payload[12..16]),
        u32_le(&payload[16..20]),
    ));
    STATUS_OK
}
