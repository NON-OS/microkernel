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

//! Lengths on the wire, as driver.hda0's `protocol/limits.rs` has them; the
//! proofs hold the two equal.

pub const STATUS_LEN: usize = 4;
/// The most PCM one WRITE_PCM carries: the audio server's piece.
pub const MAX_PCM_CHUNK: usize = 4096;
pub const OUTPUT_STATUS_PAYLOAD_LEN: usize = 12;
