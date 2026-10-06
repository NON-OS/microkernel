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

//! Endpoints only named services may reach, and what a send straight into a
//! process's own inbox must satisfy.
//!
//! A driver serves its device raw. A network card's driver puts what it is
//! sent on the wire past the stack, the route the person chose and Network,
//! and hands back every frame the card heard. A keyboard's driver hands back
//! keystrokes and, for USB, takes reports it then delivers as typed keys. The
//! USB and I2C controllers carry raw transfers to any device behind them, a
//! USB disk's sectors among them; the GPU shows whatever it is given over the
//! whole screen; the sound card plays what it is sent. So each driver is
//! reachable only by the services that drive it, named here by endpoint, and
//! the ones the kernel itself drives by none: the kernel's own sends never
//! pass this list. A caller is one of the named when it owns that endpoint,
//! which the kernel registered for it at spawn and no other capsule may claim.
//!
//! Pure: the kernel's registry supplies who owns what; the table is
//! held_table.rs. Held in kernel_proofs.

// The table sits beside this file and is mounted by path, so kernel_proofs,
// which mounts this file alone, reads the same table.
#[path = "held_table.rs"]
mod table;

use table::HELD;

/// The services that may reach `endpoint`, or None when anyone the
/// endpoint's capabilities admit may.
pub(crate) fn callers_of(endpoint: &str) -> Option<&'static [&'static str]> {
    HELD.iter().find(|(name, _)| *name == endpoint).map(|(_, callers)| *callers)
}

/// Whether a caller may send to `endpoint`, given `owns`, which says whether
/// the caller owns an endpoint of that name.
pub(crate) fn endpoint_admits(endpoint: &str, owns: impl Fn(&str) -> bool) -> bool {
    callers_of(endpoint).is_none_or(|callers| callers.iter().any(|c| owns(c)))
}

/// Whether a caller holding `held` may write straight into the inbox of a
/// process serving the endpoints `served`, each with its requirement.
///
/// Every endpoint a process serves is read from that one inbox, so a send by
/// pid is a send to each of them and must pass each one's gate: its
/// capabilities and its list. Before this, MkIpcSendToPid checked neither,
/// and a capsule without Network reached net.sockets, or a card's driver,
/// by looking up its pid.
pub(crate) fn inbox_admits<'a>(
    served: impl IntoIterator<Item = (&'a str, u64)>,
    held: u64,
    owns: impl Fn(&str) -> bool,
) -> bool {
    served
        .into_iter()
        .all(|(name, required)| held & required == required && endpoint_admits(name, &owns))
}
