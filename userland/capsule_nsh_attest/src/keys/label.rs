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

//! The names the host keys are sealed under. The machine key's label is
//! bound: the kernel derives it over nsh.attest's own manifest namespace,
//! and only for a vendor-admitted caller, so no other capsule, whatever
//! capabilities it holds, can derive the key that opens the host keys.
//! Pure, so nsh_front_proofs holds it.

/// The kernel's reserved first byte for a caller-bound label.
pub const BOUND_PREFIX: u8 = 0x01;
/// nsh.attest's name for its root, after the prefix.
pub const ROOT_NAME: &[u8] = b"nsh.host.root.v1";
/// The label's length.
pub const ROOT_LABEL_LEN: usize = 1 + ROOT_NAME.len();
/// The vault record the keys are sealed as, so the root's subkey for them
/// is theirs alone.
pub const RECORD: &[u8] = b"nsh.host.keys";

// The kernel takes at most 32 bytes after the bound prefix.
const _: () = assert!(ROOT_NAME.len() <= 32);

/// The prefix, then the name.
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
