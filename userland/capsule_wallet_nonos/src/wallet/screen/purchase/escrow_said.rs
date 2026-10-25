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

//! The sentences a made order's dispute or refund says (B8, v1.4.51 to
//! v1.4.55).

pub(super) const BAD: &str =
    "A chunk of the delivery is not what was listed. Dispute it from this \
     wallet's refund address before the window ends: the seller's bond answers for it.";
pub(super) const REFUNDABLE: &str =
    "The escrow allows a refund now. It goes to this wallet's refund \
     address, and only this machine can send it: the order's record is kept here.";
pub(super) const READING: &str = "Reading the order from the escrow.";
pub(super) const QUOTING: &str =
    "Asking settle what the transaction costs, from the refund address.";
pub(super) const GAS: &str = "The refund address needs ETH for the gas. Pay it from the Shield: a \
     standard amount, sent privately; what is left stays at that address.";
pub(super) const SIGNING: &str =
    "The Shield is signing from the refund address. Keep this window open until it is sent.";
pub(super) const SENDING: &str =
    "Sending the transaction through settle. Keep this window open until it is sent.";
pub(super) const SENT: &str = "Sent. Waiting for the escrow to record it.";
pub(super) const DISPUTED: &str = "Disputed: the seller's bond answers for it. A refund opens now.";
pub(super) const REFUNDED: &str = "Refunded to this wallet's refund address.";
