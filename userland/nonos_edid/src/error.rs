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

/// Why an EDID was refused as a whole. A bad extension block is not one
/// of these: it is skipped and the base block still answers, since a TV
/// with a corrupt CTA block still has a usable native mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Fewer than the 128 bytes of a base block.
    Short,
    /// The eight byte header 00 FF FF FF FF FF FF 00 is missing.
    Header,
    /// The base block's bytes do not sum to zero.
    Checksum,
}
