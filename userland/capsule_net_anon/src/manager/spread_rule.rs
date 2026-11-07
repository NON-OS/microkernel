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

//! The order a new stream takes the circuits that would carry it. An
//! install may hold up to four streams at once, one per chunk of an app's
//! image in flight, and each goes down a circuit carrying none of that
//! installer's other live streams while one is open, so its streams share
//! no exit and no middle relay: the warm circuit first, then the circuits
//! that have delivered, then the rest. Only when every circuit carries one
//! of them does a stream join another, in the same order. Any other stream
//! is ordered as before: delivered circuits, then the rest; never the warm
//! one (warm_rule.rs). Pure, and held in net_anon_proofs.

/// The rank of a circuit for a new stream, lowest first, or None when the
/// stream may not take it. `mine` says it already carries a live stream of
/// the same caller; it counts only for an install.
pub fn rank(install: bool, warm: bool, proven: bool, mine: bool) -> Option<u8> {
    if warm && !install {
        return None;
    }
    let within = match (warm, proven) {
        (true, _) => 0,
        (false, true) => 1,
        (false, false) => 2,
    };
    Some(if install && mine { within + 3 } else { within })
}
