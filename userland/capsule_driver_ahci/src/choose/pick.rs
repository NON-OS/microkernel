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

use super::candidate::{Candidate, Choice};

/// Serve the port whose disk carries NONOS (store header or disk plan), the
/// lowest (controller, port) first when several do. When none does, serve the
/// lowest (controller, port) that came up: the installer's blank target. The
/// answer does not depend on the order of `cands`. `None` only when no port
/// came up at all.
pub fn choose(cands: &[Candidate]) -> Option<Choice> {
    if let Some(index) = lowest(cands, |c| c.store || c.plan) {
        return Some(Choice { index, fallback: false });
    }
    lowest(cands, |_| true).map(|index| Choice { index, fallback: true })
}

fn lowest(cands: &[Candidate], keep: impl Fn(&Candidate) -> bool) -> Option<usize> {
    cands
        .iter()
        .enumerate()
        .filter(|(_, c)| keep(c))
        .min_by_key(|(_, c)| (c.controller, c.port))
        .map(|(i, _)| i)
}
