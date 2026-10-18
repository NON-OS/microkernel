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

//! The walk: every item in order under the bounds, each data field handed
//! on as it is laid out. A caller drops what it gathered when the walk is
//! refused, as a refusal can come after the last field, and what it took
//! from an input report too long to read.

use super::cursors::Cursors;
use super::global_state::GlobalState;
use super::item::next;
use super::limits::{Refused, MAX_ITEMS};
use super::locals::Locals;
use super::long::Long;
use super::main_item::Hand;

#[derive(Default)]
pub(super) struct Walk {
    pub g: GlobalState,
    pub l: Locals,
    pub depth: u8,
    /// The usage of the application (top level) collection open now.
    pub app: u32,
    /// Where each input report has got, and apart from them each feature
    /// report; None once a feature report was past a bound, after which no
    /// feature field is laid out.
    pub inputs: Cursors,
    pub features: Option<Cursors>,
    pub long: Long,
}

/// The input reports too long to read, when the descriptor is not refused.
pub(super) fn walk(desc: &[u8], f: Hand<'_>) -> Result<Long, Refused> {
    let mut w = Walk { features: Some(Cursors::default()), ..Walk::default() };
    let mut at = 0;
    for _ in 0..MAX_ITEMS {
        let Some(item) = next(desc, &mut at)? else { return w.finish().map(|()| w.long) };
        w.take(item, f)?;
    }
    if at < desc.len() {
        return Err(Refused);
    }
    w.finish().map(|()| w.long)
}
