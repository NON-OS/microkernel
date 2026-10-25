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

//! The hashes the ids and verdicts need, injected. This crate is the same
//! bytes on the host and in a capsule, so it holds no hash of its own: the
//! market passes the kernel's, nonos-mk and the tests pass a host crate's.

pub trait Keccak {
    fn keccak256(&self, data: &[u8]) -> [u8; 32];
}

pub trait Blake3 {
    fn blake3(&self, data: &[u8]) -> [u8; 32];
}
