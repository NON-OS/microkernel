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

/// PCRs covered by the quote: the firmware and boot chain measurements the
/// firmware extended. Naming them explicitly rather than quoting every PCR
/// keeps the document meaningful, since a verifier has to know which values it
/// is being shown.
pub(super) const QUOTED_PCRS: [u8; 4] = [0, 1, 2, 7];

/// Largest response this driver will accept from the part.
pub(super) const RESPONSE_MAX: usize = 4096;
