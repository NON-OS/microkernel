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

//! The meter's root: the machine key under a caller-bound label, which the
//! kernel derives over the label and the BLAKE3 of the caller's manifest
//! namespace, for Vendor-authority callers only (C2c). No other capsule can
//! derive it, capsule_linux included, though it holds Crypto.

use nonos_libc::machine_key;

use nonos_payment_proto::EAGAIN;

/// The kernel's reserved prefix byte for bound labels. C2c's letter names
/// it; until then there is no bound label to ask for, and every path that
/// needs a key answers EAGAIN. An unbound label would be a key any Crypto
/// holder could derive, so there is deliberately no fallback.
pub const BOUND_PREFIX: Option<u8> = None;

const ROOT_NAME: &[u8] = b"nonos.meter.root.v1";

pub fn root() -> Result<[u8; 32], i32> {
    let prefix = BOUND_PREFIX.ok_or(EAGAIN)?;
    let mut label = [0u8; 1 + ROOT_NAME.len()];
    label[0] = prefix;
    label[1..].copy_from_slice(ROOT_NAME);
    machine_key(&label).map_err(|e| e as i32)
}
