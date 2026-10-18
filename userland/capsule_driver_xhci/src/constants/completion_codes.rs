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
/// A command named a slot the controller does not have enabled.
pub const CC_SLOT_NOT_ENABLED: u8 = 11;
pub const CC_SHORT_PACKET: u8 = 13;
/// The Command Completion Event the controller posts once an abort stopped
/// the command ring; its pointer is the dequeue position, not a command.
pub const CC_COMMAND_RING_STOPPED: u8 = 24;
/// The answer to the command an abort gave up.
pub const CC_COMMAND_ABORTED: u8 = 25;
