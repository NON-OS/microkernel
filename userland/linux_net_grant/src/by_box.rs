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

//! A grant by the run's box: its argv, as the kernel set it when the Terminal
//! started it, names a box with destinations, which the grant carries. The
//! person typed that box, and neither the personality nor a guest can change
//! the argv. The argv is read here only to find the box, and is dropped when
//! this returns: nothing of it is logged or kept but the destinations.

use alloc::vec::Vec;

use super::{box_word, Facts, Grant, Refusal};

#[path = "../../capsule_linux/src/linux/boxed/dests.rs"]
mod box_dests;

pub fn by_box(pid: u32, facts: &impl Facts) -> Result<Grant, Refusal> {
    let argv = facts.argv(pid).ok_or(Refusal::NoBox)?;
    let items = box_word(&argv).ok_or(Refusal::NoBox)?;
    let nets: Vec<&[u8]> = items.split(|c| *c == b',').filter_map(|i| i.strip_prefix(b"net:")).collect();
    let dests = box_dests::list(&nets).map_err(|_| Refusal::NoBox)?;
    match dests.is_empty() {
        true => Err(Refusal::NoBox),
        false => Ok(Grant::Boxed { dests }),
    }
}
