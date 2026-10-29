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

//! What becomes of a NONOS disk a pass found when no disk it asked holds
//! the partition the loader came from. Pure, so kernel_proofs holds it.
//!
//! A stick boot whose USB driver found no device, or a disk no driver of the
//! kernel's drives, once kept the installed NVMe disk beside it: the boot
//! then read that disk's store, found its setup marker and came up as the
//! installed system. When the loader named its partition and read the store
//! from that same disk, the store this boot must use is the loader's copy,
//! so no other disk is kept. Nor is one when the loader came through USB,
//! whatever it named or read: a stick's boot is never the installed disk's,
//! and the loader takes no other disk's store for one (store_copy.rs). That
//! case kept the NVMe disk on an HP laptop whose firmware's paths hid the
//! stick from the loader's same-disk test.

/// The end of the pass for the disk it found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Other {
    /// Kept for the boot.
    Keep,
    /// Not yet: a disk that may hold the loader's partition has not answered.
    Wait,
    /// Never kept: the boot keeps nothing and reads the loader's copy.
    Leave,
}

/// `named`: the loader named the partition it was read from. `own_copy`:
/// it read its copy of the store from that disk. `usb`: it came through
/// USB. `refused`: a disk could not be asked this pass. `waited`: the wait
/// for such a disk has run out.
pub(super) fn other_disk(
    named: bool,
    own_copy: bool,
    usb: bool,
    refused: bool,
    waited: bool,
) -> Other {
    if refused && !waited {
        Other::Wait
    } else if usb || (named && own_copy) {
        Other::Leave
    } else {
        Other::Keep
    }
}
