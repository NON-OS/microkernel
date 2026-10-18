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

//! A claimed device leaving its holder (K10b, K10c), pure. In order: Bus
//! Master off; the holder's grants on it out of their tables; its BARs
//! unmapped from the holder, the TLB shot down; every port access already
//! past its check finished; the function reset; its DMA mappings out of the
//! IOMMU domain, the IOTLB invalidation waited on; the frames freed; the
//! domain detached; only then claimable again. A read to a function in
//! reset can end in a completion timeout, which some firmware escalates to
//! a machine check, so no BAR or port of the holder may still reach it; a
//! frame freed while a stale IOTLB entry maps it could still be written by
//! a part that ignores Bus Master.
//! kernel_proofs::function_reset.

/// The steps, each over one device of one holder.
pub(crate) trait Retire {
    fn stop_bus_master(&mut self);
    /// Its MMIO and DMA grants out of their tables, so a second path that
    /// tears the holder down meanwhile finds none to free.
    fn take_grants(&mut self);
    /// Unmapped from the holder, the shootdown complete on return.
    fn revoke_mmio(&mut self);
    /// Returns once no port access past its grant check is still running.
    fn quiet_ports(&mut self);
    fn reset(&mut self);
    /// Out of the device's domain, the IOTLB invalidation complete on return.
    fn withdraw_dma(&mut self);
    fn free_frames(&mut self);
    fn detach(&mut self);
    /// Claimable again: until here a claim on it is refused, E_BUSY.
    fn reopen(&mut self);
}

/// The one order both release paths take.
pub(crate) fn retire(io: &mut impl Retire) {
    io.stop_bus_master();
    io.take_grants();
    io.revoke_mmio();
    io.quiet_ports();
    io.reset();
    io.withdraw_dma();
    io.free_frames();
    io.detach();
    io.reopen();
}
