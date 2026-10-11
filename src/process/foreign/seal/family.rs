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

//! One live family: what the seal will say once the family ends.

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::hist::HIST_WORDS;

/// The most measurements a family's list keeps. Past it M still chains and
/// the record still counts them, so a container's list cannot rebuild M and
/// the family proves nothing: an incomplete list never passes for a whole one.
/// Distinct measurements a family keeps; past it the overflow flag is set and
/// M takes nothing more, so a guest looping maps cannot grow kernel heap.
pub(crate) const MAX_MEAS: usize = 4096;

pub(crate) struct Family {
    pub(crate) sup: u32,
    pub(crate) sup_meas: [u8; 32],
    pub(crate) sup_caps: u64,
    pub(crate) authority: u8,
    /// A first guest has been recorded. Until then the seal is only reserved.
    pub(crate) enrolled: bool,
    /// A guest of it was started; slot 0 is closed from then on.
    pub(crate) started: bool,
    pub(crate) trace: [u8; 32],
    pub(crate) rows: u64,
    pub(crate) next_seq: u64,
    pub(crate) hist: Box<[u32; HIST_WORDS]>,
    /// Some cell reached u32::MAX: the counts are lower bounds only.
    pub(crate) saturated: bool,
    pub(crate) salt: [u64; 4],
    pub(crate) meas: [u8; 32],
    pub(crate) n_meas: u32,
    pub(crate) overflow: bool,
    pub(crate) list: Vec<[u8; 32]>,
    pub(crate) app_id: Option<[u8; 32]>,
    pub(crate) diag: [u8; 32],
    /// The calls trapped and not yet answered: pid, seq and number. A
    /// thread has one at a time.
    pub(crate) open: Vec<(u32, u64, u64)>,
}
