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

//! The label the meter's root is derived under: the kernel's bound prefix
//! byte, then the meter's name for it. A bound label derives
//! over the label and the BLAKE3 of the caller's manifest namespace, and only
//! for a Vendor-authority caller with a recorded namespace; anyone else gets
//! EPERM. So no other capsule can derive the meter's root, capsule_linux
//! included, though it holds Crypto. The bound part may be at most 32 bytes.

/// The kernel's reserved first byte for caller-bound labels (C2c).
pub const BOUND_PREFIX: u8 = 0x01;

/// The meter's name for its root, after the prefix.
pub const ROOT_NAME: &[u8] = b"nonos.meter.root.v1";

pub const ROOT_LABEL_LEN: usize = 1 + ROOT_NAME.len();

const _: () = assert!(ROOT_NAME.len() <= 32);

/// 0x01 | "nonos.meter.root.v1", 20 bytes.
pub const fn root_label() -> [u8; ROOT_LABEL_LEN] {
    let mut label = [0u8; ROOT_LABEL_LEN];
    label[0] = BOUND_PREFIX;
    let mut i = 0;
    while i < ROOT_NAME.len() {
        label[1 + i] = ROOT_NAME[i];
        i += 1;
    }
    label
}
