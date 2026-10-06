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

//! WPA3-Personal: Simultaneous Authentication of Equals (IEEE Std
//! 802.11-2020, 12.4) over group 19 (NIST P-256), station side. The password
//! element comes from hash-to-element (`h2e`) when the access point's RSNXE
//! offers it and from hunting and pecking (`hnp`) otherwise; `commit` builds
//! the commit and derives KCK, PMK and PMKID; `frame` encodes and parses the
//! Authentication frame bodies; `station` runs the exchange. The PMK then
//! keys an ordinary four-way handshake under the SAE AKM (00-0F-AC:8). Every
//! derivation is checked against the IEEE Std 802.11-2020 Annex J.10 vectors
//! in `nonos_wifi_core_proofs`.

pub mod commit;
pub mod frame;
pub mod group;
pub mod h2e;
pub mod hnp;
pub mod station;

pub use station::{SaeFailure, SaeState, SaeStation, SaeStep};

/// Overwrite secret intermediate bytes so they do not linger on the stack.
pub(crate) fn wipe(buf: &mut [u8]) {
    for b in buf.iter_mut() {
        // SAFETY: `b` is a valid, aligned, exclusive reference into `buf`.
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
