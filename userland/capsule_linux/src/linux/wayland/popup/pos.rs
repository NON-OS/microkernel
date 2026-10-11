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

//! xdg_positioner's requests, kept until the positioner is destroyed.

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::positioner::{set, Positioner};

/// A positioner's set_* request; one xdg-shell calls invalid_input leaves
/// it as it was.
pub fn position(guest: &mut Guest, id: u32, op: u16, args: &mut Args<'_>) {
    let mut words = [0u32; 4];
    let n = (0..4).take_while(|i| args.u32().map(|w| words[*i] = w).is_some()).count();
    let rules = &mut guest.scene.popups.positioners;
    if !rules.iter().any(|(p, _)| *p == id) {
        rules.push((id, Positioner::default()));
    }
    if let Some((_, rule)) = rules.iter_mut().find(|(p, _)| *p == id) {
        let _ = set(rule, op, &words[..n]);
    }
}

/// The positioner `id` destroyed.
pub fn forget(guest: &mut Guest, id: u32) {
    guest.scene.popups.positioners.retain(|(p, _)| *p != id);
}
