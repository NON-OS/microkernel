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

//! TLS record and inner content types.

pub(super) const CHANGE_CIPHER_SPEC: u8 = 20;
pub(super) const ALERT: u8 = 21;
pub(super) const HANDSHAKE: u8 = 22;
pub(super) const APPLICATION_DATA: u8 = 23;

pub(super) const FINISHED: u8 = 20;
pub(super) const KEY_UPDATE: u8 = 24;
