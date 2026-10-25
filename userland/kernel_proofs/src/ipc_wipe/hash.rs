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

//! The checksum without the IPC secret: these proofs are about the bytes'
//! lifetime, not the envelope's integrity.

pub fn compute_checksum(
    from: &str,
    to: &str,
    data: &[u8],
    ts_ms: u64,
) -> Result<u64, &'static str> {
    Ok((from.len() + to.len() + data.len()) as u64 ^ ts_ms)
}

pub(super) fn verify_checksum(from: &str, to: &str, data: &[u8], ts_ms: u64, sum: u64) -> bool {
    compute_checksum(from, to, data, ts_ms) == Ok(sum)
}
