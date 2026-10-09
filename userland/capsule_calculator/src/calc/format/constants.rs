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

// The widest value a Fixed can hold: a sign, 31 integer digits (i128::MIN
// over FRAC), the point and 8 fraction digits, 41 bytes. At 32 the fraction
// and then the low digits of a large result were cut off and a different
// number shown; the readout's font ladder fits the full width instead.
pub const DISPLAY_MAX: usize = 48;
