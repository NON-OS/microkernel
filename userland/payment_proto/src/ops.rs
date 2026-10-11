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

/// The meter's registry name.
pub const SERVICE: &[u8] = b"payment";

pub const OP_HEALTHCHECK: u16 = 1;
pub const OP_PAY: u16 = 2;
pub const OP_DRAIN: u16 = 3;
pub const OP_LIST_TOKENS: u16 = 4;
pub const OP_ACCOUNT: u16 = 5;
pub const OP_SET_BUDGET: u16 = 6;
/// Reserved: NXL1 is not in v1, and the meter answers EINVAL.
pub const OP_ENTITLE: u16 = 7;
pub const OP_ORDER_NEW: u16 = 8;
pub const OP_ORDER_STATUS: u16 = 9;
pub const OP_ORDER_DELIVERED: u16 = 10;
pub const OP_ORDER_REFUND: u16 = 11;
pub const OP_OPEN: u16 = 12;
