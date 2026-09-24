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

pub fn expand_label(prk: &[u8; 32], label: &[u8], context: &[u8], out: &mut [u8]) -> bool {
    // The structure is built by `hkdf_label`, which is pure and has proofs; this
    // is the expansion it feeds, which is a syscall and has none.
    match super::hkdf_label::hkdf_label(out.len(), label, context) {
        Some(info) => super::hkdf::expand(prk, &info, out),
        None => false,
    }
}
