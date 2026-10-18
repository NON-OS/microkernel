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

//! A refused or failed region access: counted, and told the first time
//! for its (space, region base), with the reason `spaces` gives.

use super::spaces::path;
use super::state::CapsuleHost;
use super::told::{tell, Tell};
use crate::sys::log::Line;

pub(super) fn refuse(h: &mut CapsuleHost, space: u8, base: u64) {
    h.refused += 1;
    let verdict = tell(&mut h.told, (space, base));
    if verdict != Tell::Quiet {
        let l = Line::new(b"region space ").hex(u64::from(space)).str(b" at ").hex(base);
        let l = l.str(b" refused: ").str(path(space).1.as_bytes());
        if verdict == Tell::Last { l.str(b"; further refusals are counted, not told") } else { l }
            .end();
    }
}
