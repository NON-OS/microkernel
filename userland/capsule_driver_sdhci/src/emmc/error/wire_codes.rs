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

//! The bases of the request statuses that carry host and card detail.

/// The status a failed request answers its client. Timeouts are -110. A
/// host error is -(HOST_ERROR_BASE | CMD << 16 | Error Interrupt Status), a
/// card that ended the command with an R1 error is -(CARD_ERROR_BASE | CMD
/// << 16 | the highest R1 error bit's number), so a failed install can say
/// what the host or the card said. Anything else is an I/O error (-5).
pub const HOST_ERROR_BASE: i32 = 0x100_0000;
pub const CARD_ERROR_BASE: i32 = 0x200_0000;
