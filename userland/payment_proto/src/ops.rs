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
/// Wallet: approve and topUp a channel with the meter's key (TopUp).
pub const OP_TOPUP: u16 = 13;
/// Wallet: close a channel past expiry plus GRACE (Close).
pub const OP_CLOSE: u16 = 14;
/// Wallet: a started job's result, once (job.rs).
pub const OP_JOB: u16 = 15;
/// Wallet: claim the NOX the Shield pool credited the meter (v1.4.17).
/// Body: chain_id u64. A job; its result is claimTx.
pub const OP_CLAIM: u16 = 16;
/// Wallet: fetch, open and keep an escrow offer (plan/29). A job; its
/// result is offerId | cipherRoot | keyCommit.
pub const OP_ORDER_OFFER: u16 = 17;
/// app.linux.install: an order's checked plaintext, orderId | offset u64,
/// up to PLAINTEXT_PAGE bytes a reply (plan/29).
pub const OP_ORDER_PLAINTEXT: u16 = 18;
/// The store-to-wallet mailbox: 19/0 POST from app.store, 19/1 TAKE from
/// the wallet (plan/21, plan/27).
pub const OP_PURCHASE: u16 = 19;
/// One listing's cap, the rest of the budget kept (v1.4.43, wallet).
pub const OP_LISTING_CAP: u16 = 20;
