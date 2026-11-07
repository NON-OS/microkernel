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

use nonos_libc::{delivery_window, DELIVERY_HDR_LEN, DELIVERY_LEN_V1, DELIVERY_MAGIC};

use super::InputEvent;

/// An input router delivery frame read whole: the event, and the window of
/// the receiver's it was routed to when the frame names one (a pointer
/// event over one of several windows, its x and y local to that window).
/// None for a frame that is not a delivery or an event of an unknown kind.
pub fn parse_routed(frame: &[u8]) -> Option<(InputEvent, Option<u32>)> {
    if frame.len() < DELIVERY_LEN_V1 {
        return None;
    }
    if u32::from_le_bytes(frame[0..4].try_into().ok()?) != DELIVERY_MAGIC {
        return None;
    }
    let event = InputEvent::from_delivery(&frame[DELIVERY_HDR_LEN..])?;
    Some((event, delivery_window(frame)))
}
