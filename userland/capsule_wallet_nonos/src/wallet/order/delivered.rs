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

/* What a delivery's check found, in one line (B8, the plan), and when the
 * meter is asked for it. */

use alloc::format;
use alloc::string::String;

use nonos_payment_proto::{DeliveredMade, OrderStatus};

const DELIVERED: u8 = 2;

/// Whether ORDER_DELIVERED is asked: once orderOf reads Delivered and the
/// plaintext is not sealed, and never after an answer, OK or a bad chunk
/// (v1.4.49): the meter keeps it, and a bad chunk waits for the Dispute.
pub fn delivery_due(s: Option<&OrderStatus>, made: Option<&DeliveredMade>) -> bool {
    made.is_none() && s.is_some_and(|s| s.state == DELIVERED && !s.plain)
}

pub fn delivered_line(d: &DeliveredMade) -> String {
    match d {
        DeliveredMade::Ok { content_len } => {
            format!("every chunk checked: {content_len} bytes ready to install")
        }
        DeliveredMade::BadChunk { index, .. } => {
            format!("chunk {index} is not what was listed: dispute it before the window ends")
        }
    }
}
