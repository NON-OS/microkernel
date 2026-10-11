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

//! Why a refund is not sent (B8, kind 11), in the person's words.

pub const ALREADY: &str = "The escrow has refunded this order already.";
pub const NOTHING_LEFT: &str = "Nothing of this order is left in the escrow to refund.";
pub const NOT_YET: &str = "The seller has until the delivery deadline; a refund opens then.";
pub const IN_WINDOW: &str = "The order was delivered: only a dispute inside its window refunds it.";
