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
//! The keys an answer for the fixture's offer is sealed under: the buyer's,
//! the agent's, and the listing and release it is bound to.

pub const BUYER: [u8; 32] = [0x0b; 32];
pub const AGENT: [u8; 32] = [0x0a; 32];
pub const LISTING: [u8; 32] = [0x11; 32];
pub const RELEASE: [u8; 32] = [0x22; 32];
