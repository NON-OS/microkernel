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

use super::ask::ask;

const CALL_TIMEOUT_MS: u64 = 2000;

pub fn call(port: u32, magic: u32, op: u16, body: &[u8], rx: &mut [u8]) -> Result<usize, ()> {
    call_t(port, magic, op, body, rx, CALL_TIMEOUT_MS)
}

pub fn call_t(
    port: u32,
    magic: u32,
    op: u16,
    body: &[u8],
    rx: &mut [u8],
    timeout_ms: u64,
) -> Result<usize, ()> {
    ask(port, magic, op, body, rx, timeout_ms).map_err(|_| ())
}
