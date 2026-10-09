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

/*
 * One delivery from the input router as the event the window's key router
 * takes. A line feed is Enter, as setup reads it.
 */

use nonos_app_skeleton::wire::NINP_MAGIC;
use nonos_app_skeleton::{InputEvent, KEY_ENTER};

const HDR_LEN: usize = 8;
pub(super) const DELIVERY_LEN: usize = HDR_LEN + 32;
const KEY_LINE_FEED: u32 = 0x0A;

pub(super) fn parse(buf: &[u8]) -> Option<InputEvent> {
    if buf.len() < DELIVERY_LEN || buf[0..4] != NINP_MAGIC.to_le_bytes() {
        return None;
    }
    let mut e = InputEvent::from_delivery(&buf[HDR_LEN..])?;
    if e.code == KEY_LINE_FEED {
        e.code = KEY_ENTER;
    }
    Some(e)
}
