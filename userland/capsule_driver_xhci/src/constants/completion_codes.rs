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
pub const CC_SUCCESS: u8 = 1;
/// A bus error on one transaction; for an isoch packet, that packet is lost.
pub const CC_USB_TRANSACTION: u8 = 4;
pub const CC_SHORT_PACKET: u8 = 13;
/// An isoch endpoint found its ring empty (OUT) or its buffer full (IN):
/// no TD names it, so no waiter ever asks for the event.
pub const CC_RING_UNDERRUN: u8 = 14;
pub const CC_RING_OVERRUN: u8 = 15;
/// An isoch TD whose service interval had passed when it was reached.
pub const CC_MISSED_SERVICE: u8 = 23;
