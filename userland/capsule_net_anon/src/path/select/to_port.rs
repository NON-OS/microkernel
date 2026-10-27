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

//! Drawing the exit for a stream to one port.

use super::super::draw::pick;
use super::super::relay::Relay;
use super::super::weights::{Position, Weights};
use super::candidates::candidates_where;
use super::eligible::{eligible_by, Taken};

/// Draw an exit that `takes` the stream, `None` when no relay does. It is an
/// exit `eligible` would take with `takes` in place of the web ports,
/// weighted and kept apart from `taken` and their /16 as any exit is. With
/// `takes` the web rule it draws exactly what `choose` draws for an exit.
pub fn choose_exit<'a>(
    relays: &'a [Relay],
    weights: &Weights,
    taken: &[Taken],
    takes: impl Fn(&Relay) -> bool,
    roll: u64,
) -> Option<&'a Relay> {
    let fits = |relay: &Relay| eligible_by(relay, Position::Exit, &takes);
    let (chosen, scaled) = candidates_where(relays, weights, Position::Exit, taken, fits);
    chosen.get(pick(&scaled, roll)?).copied()
}

/// Whether any relay could be an exit that `takes` the stream, before the
/// guard is known. When none could, a stream does not wait for one.
pub fn any_exit(relays: &[Relay], weights: &Weights, takes: impl Fn(&Relay) -> bool) -> bool {
    choose_exit(relays, weights, &[], takes, 0).is_some()
}
