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

//! A claimed device leaving its holder, pure: Bus Master off, the
//! holder's grants on it out of their tables, its BARs unmapped from the
//! holder with the TLB shot down, the function reset, then its DMA frames
//! freed. A read to a function in reset can end in a completion timeout,
//! which some firmware escalates to a machine check, so no holder thread
//! may still reach a BAR; and no frame is reused before the reset stops
//! what a dead driver left armed. kernel_proofs::function_reset.

/// The steps, each over one device of one holder.
pub(crate) trait Retire {
    fn stop_bus_master(&mut self);
    /// Its MMIO and DMA grants out of their tables, so a second path that
    /// tears the holder down meanwhile finds none to free.
    fn take_grants(&mut self);
    /// Unmapped from the holder, the shootdown complete on return.
    fn revoke_mmio(&mut self);
    fn reset(&mut self);
    fn free_frames(&mut self);
}

/// The one order both release paths take.
pub(crate) fn retire(io: &mut impl Retire) {
    io.stop_bus_master();
    io.take_grants();
    io.revoke_mmio();
    io.reset();
    io.free_frames();
}
