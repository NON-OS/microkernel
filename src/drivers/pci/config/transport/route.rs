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

//! Which mechanism reaches one (segment, bus, offset). Pure, so the host
//! proofs hold it.
//!
//! An ECAM window wins whenever one covers the bus. Without one only a PC has
//! another way in, the 0xCF8 port pair, and that reaches segment 0 below
//! offset 0x100 alone. Anything else is unreachable and says so: config reads
//! answer all-ones for an absent device, so falling back silently would
//! enumerate an empty bus and look like a machine with nothing on it.

use super::ecam::EcamWindow;

/// The port pair's reach: 256 bytes of one function.
pub(super) const LEGACY_LIMIT: u16 = 0x100;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Route {
    Ecam(EcamWindow),
    Legacy,
    Unreachable,
}

/// `window` is the registry's answer for (segment, bus); `ports` is whether
/// this machine has the port pair at all.
pub(super) const fn route(
    window: Option<EcamWindow>,
    ports: bool,
    segment: u16,
    offset: u16,
) -> Route {
    match window {
        Some(w) => Route::Ecam(w),
        None if ports && segment == 0 && offset < LEGACY_LIMIT => Route::Legacy,
        None => Route::Unreachable,
    }
}
