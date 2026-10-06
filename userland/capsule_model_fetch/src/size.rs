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

/* A size in the unit a person reads. Pure, so the host proofs share it. */

/* `bytes` as "491 MB" or "4.68 GB". */
pub fn size(bytes: u64) -> alloc::string::String {
    match bytes {
        b if b >= 1_000_000_000 => {
            alloc::format!("{}.{:02} GB", b / 1_000_000_000, b / 10_000_000 % 100)
        }
        b if b >= 1_000_000 => alloc::format!("{} MB", b / 1_000_000),
        b => alloc::format!("{} KB", b / 1_000),
    }
}
