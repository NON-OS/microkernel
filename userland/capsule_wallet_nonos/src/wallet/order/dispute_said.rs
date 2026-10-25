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

//! Why a dispute is not sent (B8, kind 10), in the person's words.

pub const NOTHING: &str = "Every chunk checked out: there is nothing to dispute.";
pub const NOT_REFUND_TO: &str =
    "A dispute is sent from the order's refund address and no other, not signing.";
pub const NOT_THIS_ORDER: &str =
    "The meter's dispute is not a dispute of this order's terms, not signing.";
pub const NOT_DELIVERED: &str =
    "The escrow does not read Delivered, so there is no window to dispute in yet.";
pub const TOO_LATE: &str = "Less than an hour of the dispute window is left, too little for the \
     transaction to be mined in time; the seller is paid when it ends.";
