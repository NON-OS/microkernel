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

//! What a controller asks for, read from Identify Controller bytes 272 to
//! 279 and 332 to 337 (ControllerIdentity::parse).

/// What the controller's identify page says about host memory, in pages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HmbAsk {
    /// HMPRE: the size it prefers. Zero: it wants none.
    pub preferred: u32,
    /// HMMIN: the least it can use.
    pub minimum: u32,
    /// HMMINDS: the smallest piece it accepts; zero means no limit.
    pub min_piece: u32,
    /// HMMAXD: the most pieces; zero means no limit.
    pub max_pieces: u16,
}
