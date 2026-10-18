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

//! What names a device, kept only as a digest: its INQUIRY and its serial
//! number are compared with the device served (scan/identity.rs), never
//! stored, logged or returned.

/// FNV-1a over `bytes`.
pub(super) fn digest(bytes: &[u8]) -> u64 {
    let step = |h: u64, &b: &u8| (h ^ u64::from(b)).wrapping_mul(0x100_0000_01b3);
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, step)
}
