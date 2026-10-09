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

use crate::image::types::DecodeError;

/* How the coefficient decoder turns streams away, named for the reason. */

/// A stream that breaks the structure T.81 requires: a scan before its
/// frame, a table or component id never defined, a band out of range. The
/// bytes do not form a JPEG stream, which is what BadMagic reports.
pub const MALFORMED: DecodeError = DecodeError::BadMagic;

/* A frame coded with a process other than 8-bit Huffman baseline,
 * extended or progressive (lossless, hierarchical, arithmetic, 12-bit):
 * the same refusal the header parser gives such a frame. */
pub use crate::image::jpeg::decode::OTHER_PROCESS;

/// Entropy-coded data that stops making sense (an impossible code, or a
/// coefficient past its band): the valid data ends there, exactly as if the
/// stream were cut at that point, so it is reported as Truncated and the
/// scan keeps what it decoded, as libjpeg shows a partly damaged picture.
pub const CORRUPT_DATA: DecodeError = DecodeError::Truncated;
