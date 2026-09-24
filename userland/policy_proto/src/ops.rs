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

pub const OP_GET: u16 = 0x0001;
pub const OP_SET: u16 = 0x0002;

/*
 * Addresses no field, unlike GET and SET: it asks what the kernel reports about
 * its own hardening, which is one record rather than a value per row, so the
 * header's field word is unused on both sides.
 */
pub const OP_STATUS: u16 = 0x0003;
