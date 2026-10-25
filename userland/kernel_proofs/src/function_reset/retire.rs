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

//! A claimed device leaving its holder, pure: the order K10
//! built, kept here as a model. Bus Master off; the holder's grants out of
//! their tables; its BARs unmapped from the holder, the TLB shot down; every
//! port access past its check finished; the reset; DMA out of the IOMMU
//! domain, the invalidation waited on; the frames freed; the domain
//! detached; then claimable. The kernel keeps the order (give_back.rs,
//! claim/release.rs, claim/wipe): every grant drained, pages out of the
//! holder's tables and DMA out of the domain before a frame is freed, and the
//! reset as the wipe once the entry is gone, so the properties the fake
//! checks inside `reset` and `free` hold there too.

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
