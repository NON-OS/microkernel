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

//! Which peers a capsule may reach over IPC, for the capsules that are held
//! to a list.
//!
//! A capability says what kind of thing a capsule may do; it cannot say who
//! it may do it with. A capsule on this list reaches the endpoints named for
//! it and nothing else, whatever its capabilities admit, and a capsule not
//! on it is unaffected. The table lives in the kernel image, so it is
//! measured with it, and the capsule format stays as it is.

/// Capsule name, then the endpoint names it may send to.
const PEERS: &[(&str, &[&str])] = &[
    // The Shield prover holds a witness. Its one peer is the core that sent
    // it, so it reaches no network, no storage and no other capsule.
    ("shield_prover", &["shield.core"]),
];

/// The list a capsule is held to, or None when it is not held to one.
pub(crate) fn peers_of(caller: &str) -> Option<&'static [&'static str]> {
    PEERS.iter().find(|(name, _)| *name == caller).map(|(_, peers)| *peers)
}

/// Whether `caller` may send to the endpoint called `target`.
pub(crate) fn may_reach(caller: &str, target: &str) -> bool {
    peers_of(caller).is_none_or(|peers| peers.contains(&target))
}
