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

//! The spends kept before the last, as a follow's reply lists them.

use alloc::vec::Vec;

use shield_wire::{field, fields};

/* One kept spend, as one `earlier` value. */
pub struct Kept<'a> {
    pub id: &'a str,
    pub state: &'a str,
    pub minutes: u32,
    pub self_settle: bool,
    pub tx: &'a str,
}

fn kept(line: &str) -> Option<Kept<'_>> {
    let mut p = line.split('|');
    let id = p.next()?;
    let state = p.next()?;
    let minutes = p.next()?.parse().ok()?;
    let self_settle = p.next()? == "1";
    let tx = p.next()?;
    Some(Kept { id, state, minutes, self_settle, tx })
}

/* Every kept spend the follow saw, oldest first; None when the service
 * could not read them this time, so nothing is let go on a failed read. */
pub fn kept_all(body: &str) -> Option<Vec<Kept<'_>>> {
    if field(body, "earlier_why").is_some() {
        return None;
    }
    Some(fields(body, "earlier").filter_map(kept).collect())
}
