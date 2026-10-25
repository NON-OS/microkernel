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

//! When firmware's DMA protection may be let go (plan/50 P4). Pure, and
//! proven on the host (kernel_proofs::iommu_handover).
//!
//! Firmware may leave VT-d protected memory regions or translation on, or an
//! AMD-Vi unit running its own tables. Each is protection. It is let go only
//! once the kernel's own tables are live, so there is never a moment between
//! the two. A bring-up that fails, or one never built in, keeps it: claims
//! are then refused (broker/confine/decide.rs). The owner's development
//! opt-in to unconfined DMA lets it go at probe, so drivers' DMA works,
//! unless firmware set the DMAR platform opt-in flag, which keeps it.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Moment {
    /// At probe, before the kernel's tables exist: the development opt-in only.
    AtProbe,
    /// Once every unit runs under the kernel's tables; never on a failure.
    AfterTables,
    Never,
}

/// For VT-d. `enforce`: bring-up is built in. `dev`: the unconfined-DMA
/// opt-in holds. `platform`: the DMAR platform opt-in flag is set.
pub const fn vtd(enforce: bool, dev: bool, platform: bool) -> Moment {
    match (enforce, dev && !platform) {
        (true, _) => Moment::AfterTables,
        (false, true) => Moment::AtProbe,
        (false, false) => Moment::Never,
    }
}

/// For AMD-Vi, which has no protected regions: a unit firmware left running
/// is stopped only to be brought up with the kernel's tables (`backend`, the
/// driver built in), or under the development opt-in.
pub const fn amd_stop(backend: bool, dev: bool) -> bool {
    backend || dev
}
