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

//! Settle's BROADCAST answer when it names another hash than the one the
//! wallet signed and follows: on its own line under the hash followed, as
//! a warning (intent::echo_said), then the node's hash whole. Never the
//! primary hash, never followed.

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::parts::value::value_block;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::{BAD, GAP};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::hex_id::hex;
use crate::wallet::intent::echo_said;

/// Draw the node's answer from `y` in `c`, when there is one; its height.
pub fn echo(fb: &mut PaintBuffer, c: Rect, y: u32, echo: Option<[u8; 32]>) -> u32 {
    let (Some(e), Some(said)) = (echo, echo_said(echo)) else { return 0 };
    let (x, w) = (c.x as i32, c.w as i32);
    let h = wrapped(fb, x, y as i32, w, Role::Fact, &said, BAD).max(0) as u32 + GAP / 2;
    h + value_block(fb, c.x, y + h, c.w, &hex(&e)) + GAP
}
