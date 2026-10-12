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

//! The hard bounds on a report descriptor walk. A descriptor past any of
//! them is refused whole and its interface is left unbound.

/// A descriptor the walk refuses: past a bound, cut inside an item, or with
/// a collection left open. Which of them is not kept, as the interface is
/// left unbound whichever.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refused;

/// Items walked. A descriptor is read in one control transfer of at most
/// 512 bytes, so a real one never comes near it.
pub const MAX_ITEMS: usize = 512;
/// Collections open at once.
pub const MAX_DEPTH: u8 = 8;
/// Global states saved by Push and not yet restored.
pub const MAX_PUSH: usize = 4;
/// Usages kept for one main item; fields past them take the last one's.
pub const MAX_USAGES: usize = 32;
/// Input reports, by report ID, one interface may declare.
pub const MAX_REPORTS: usize = 8;
/// The widest field a Report Size may give, as Linux allows.
pub const MAX_FIELD_BITS: u32 = 256;
/// One main item's report size times count, and one input report's bits.
pub const MAX_REPORT_BITS: u32 = 2048;
