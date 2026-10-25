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

//! NCMP 0x0012 OUTPUT_NOTICE: the display driver now driven says an output
//! went dark or came back, or that its device left (`display::notice_rule`).
//! Never answered, from anyone: a notice elicits nothing, so no client can
//! use it to make the compositor send.

use crate::display::notice_rule::heed;
use crate::state::Context;

pub fn handle(ctx: &mut Context, sender: u32, body: &[u8]) -> Result<(), &'static str> {
    if let Some(link) = ctx.wire.as_mut() {
        heed(link, sender, body);
    }
    Ok(())
}
