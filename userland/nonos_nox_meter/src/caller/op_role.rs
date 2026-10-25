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

//! Each meter op with the role its caller must play, read before
//! the body is: the capsule's dispatch asks here, then `Role::admits`. Op 7
//! (NXL1) is not in v1; it and any op not listed are None, EINVAL.

use nonos_payment_proto::{OP_ACCOUNT, OP_DRAIN, OP_FUNDED, OP_HEALTHCHECK, OP_LIST_TOKENS};
use nonos_payment_proto::{OP_CLAIM, OP_CLOSE, OP_JOB, OP_OPEN, OP_PAY, OP_SET_BUDGET, OP_TOPUP};
use nonos_payment_proto::{OP_LISTING_CAP, OP_ORDER_PLAINTEXT, OP_ORDER_REFUND, OP_ORDER_STATUS};
use nonos_payment_proto::{OP_ORDER_DELIVERED, OP_ORDER_NEW, OP_ORDER_OFFER, OP_PURCHASE};
use nonos_payment_proto::{PURCHASE_POST, PURCHASE_READY, PURCHASE_TAKE, PURCHASE_TAKE_READY};

use super::role::Role;

pub fn op_role(op: u16, body: &[u8]) -> Option<Role> {
    Some(match op {
        OP_HEALTHCHECK | OP_LIST_TOKENS => Role::Anyone,
        // JOB answers only the pid that started the job (Jobs::take).
        OP_PAY | OP_ORDER_STATUS | OP_JOB => Role::Payer,
        OP_DRAIN => Role::Settle,
        OP_ACCOUNT | OP_SET_BUDGET | OP_OPEN | OP_TOPUP | OP_CLOSE | OP_CLAIM => Role::Wallet,
        OP_ORDER_NEW | OP_ORDER_REFUND | OP_ORDER_OFFER | OP_LISTING_CAP => Role::Wallet,
        OP_ORDER_DELIVERED => Role::Deliverer,
        OP_ORDER_PLAINTEXT => Role::Install,
        // The store's one bit before it posts (pay::funded); ACCOUNT's rows
        // stay the wallet's.
        OP_FUNDED => Role::Store,
        // 19/0 the store's POST, 19/1 the wallet's TAKE.
        OP_PURCHASE => return purchase_role(body),
        _ => return None,
    })
}

/// Who may send this PURCHASE body: the store posts names and takes the
/// orderId; the wallet takes names and posts the orderId.
pub fn purchase_role(body: &[u8]) -> Option<Role> {
    match *body.first()? {
        PURCHASE_POST | PURCHASE_TAKE_READY => Some(Role::Store),
        PURCHASE_TAKE | PURCHASE_READY => Some(Role::Wallet),
        _ => None,
    }
}
