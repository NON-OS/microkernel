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

pub(super) const MAX_PORTS: usize = 255;
pub(super) const PORTSC_CONNECTED: u32 = 1;
/// A port's owner byte in the controller driver's port status.
pub(super) const PORT_FREE: u8 = 0;
/// Tries of one port that come to nothing before it is left alone.
pub(super) const TRIES: u8 = 3;
