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

/* What a delivery's check found, in one line (B8, the plan). */

use alloc::format;
use alloc::string::String;

use nonos_payment_proto::DeliveredMade;

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
