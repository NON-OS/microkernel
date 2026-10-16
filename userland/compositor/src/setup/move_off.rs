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

//! Off a driver whose device left (OUTPUT_NOTICE, LEAVING): the best other
//! driver announced now, in the kernel's list order, primed and adopted, the
//! first that primes. With none, the desktop stays where it is, waiting as
//! for a driver that vanished, and each poll looks again.

use crate::state::Context;

use super::discover::{displays::DISPLAYS, lookup_at};

pub fn move_off(ctx: &mut Context, gone: u32) -> Option<&'static str> {
    let others = (0..DISPLAYS.len()).filter_map(lookup_at).filter(|e| e.pid != gone);
    let next = others.into_iter().find_map(|e| super::prime_wire::prime(e).ok())?;
    super::adopt::adopt(ctx, next);
    Some("its device left; moved to the next display")
}
