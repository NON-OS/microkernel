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

/// Why a window was not published. Each is the producer's to report: the
/// buses it described stay unreachable through ECAM.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EcamError {
    /// Every slot is taken.
    Full,
    /// It claims a bus a published window already claims.
    Overlaps,
    /// No buses, or no mapping.
    Empty,
}
