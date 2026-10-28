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

//! A handover: the data volume's key wrapped once more, so the first boot
//! of the next release's bootloader opens it.
//!
//! Way 3 wraps the volume key under a key the TPM gives a kernel the release
//! approved, on this machine's firmware, bootloader and Secure Boot state
//! (PCR 0, 4 and 7). PCR 4 is the bootloader itself, which most releases
//! build anew, so an update alone would leave the volume closed. Before one,
//! the person asks the installer to prepare (prepare.rs): the key is wrapped
//! again under the handover key, the same policy without PCR 4, into the key
//! header's sector (place.rs). The next boot opens the volume under its own
//! key if it can and clears the record; if it cannot, the record opens it,
//! the key is wrapped under this boot's key and the record cleared
//! (moved.rs). A handover lasts one boot either way.
//!
//! What it costs: from the prepare to that boot, a kernel the release
//! approved opens the volume on this machine whatever bootloader measured
//! it. With Secure Boot off, a bootloader that lies about PCR 9 could then
//! start another kernel that does; a copy of the disk taken in that window
//! keeps the record. This is said where the person prepares, and in
//! docs/security/measured-boot-and-tpm.md.

mod layout;
mod moved;
mod place;
mod prepare;
mod rewrap;
mod seal;
mod sector;

pub(super) use moved::opened;
pub use prepare::prepare_handover;

/// Why a prepare was refused. Nothing was written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandoverRefusal {
    /// This boot keeps nothing: no volume on a disk to hand over.
    NotKept,
    /// The volume is not open yet.
    NotOpen,
    /// A passphrase keys the volume, and it opens under any release.
    NoNeed,
    /// Way 1, or no key header: a key derived for one kernel is not wrapped.
    CannotKeep,
    /// This boot's chain is not one the release approved, or its key does
    /// not open the volume.
    NotApproved,
}
