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

//! What happens to one leaf frame of a dying address space, as a pure rule.
//! A frame the process held alone is scrubbed, then freed: freed as it was,
//! it carried what the process held past its exit and past the power-off
//! wipe, which reaches only live processes. A frame another process still
//! holds, or a device window the broker scrubs and frees itself, is left.
//!
//! No crate paths here, so the host proofs mount this file as it is.

/// Who a leaf frame belongs to once its process is gone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Leaf {
    Own,
    Kept,
    Device,
}

pub(crate) fn leaf(kept: bool, device: bool) -> Leaf {
    match (kept, device) {
        (true, _) => Leaf::Kept,
        (_, true) => Leaf::Device,
        _ => Leaf::Own,
    }
}

/// Scrub, then free, a frame the process held alone; leave any other.
pub(crate) fn release(leaf: Leaf, zero: impl FnOnce(), free: impl FnOnce()) {
    if leaf == Leaf::Own {
        zero();
        free();
    }
}

/// Whether the power-off wipe scrubs a live process's leaf frame: every one
/// but a device window, whose bytes are registers, not memory; the broker
/// wipes the DMA frames it granted on its own.
pub(crate) fn scrubbed_at_end(leaf: Leaf) -> bool {
    leaf != Leaf::Device
}
