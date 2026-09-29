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

//! How long each hop gets to answer.

/*
 * One hop is one round trip to a relay that may be on another continent, and
 * the third hop's answer has to cross all three. Generous rather than tight: a
 * circuit abandoned early costs three handshakes and the next attempt pays them
 * again.
 */
pub(super) const HOP_MS: i64 = 15_000;
