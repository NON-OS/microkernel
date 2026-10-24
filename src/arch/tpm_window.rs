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

//! Where the TPM's register window is, asked of the platform rather than
//! assumed: a PC places it at one fixed address, an Arm board wherever its
//! device tree says.

/// The window's physical base and length, or `None` where no TPM register
/// window is known.
pub(crate) fn tpm_window() -> Option<(u64, u64)> {
    // The PC Client platform's fixed TPM range: five 4 KiB localities from
    // 0xFED40000 (TCG PC Client Platform TPM Profile; recalled, unverified).
    #[cfg(target_arch = "x86_64")]
    return Some((0xFED4_0000, 0x5000));
    #[cfg(target_arch = "aarch64")]
    return crate::arch::aarch64::tpm_tree::get();
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    None
}
