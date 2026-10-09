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

//! The variants the enroll tool wrote, each from an honest enrollment changed
//! afterwards, and the probe identity that grants one capability more.

macro_rules! variant {
    ($name:literal) => {
        include_bytes!(concat!("../../../target/attest-refusal/", $name))
    };
}

pub(super) const FLIP_TRAILER: &[u8] = variant!("flip.trailer");
pub(super) const KERNEL_KIND_TRAILER: &[u8] = variant!("kernel_kind.trailer");
pub(super) const STALE_EPOCH_TRAILER: &[u8] = variant!("stale_epoch.trailer");
pub(super) const EXTRA_CAP_CERT: &[u8] = variant!("extra_cap.nonos_id_cert.bin");
pub(super) const EXTRA_CAP_MANIFEST: &[u8] = variant!("extra_cap.manifest.bin");
