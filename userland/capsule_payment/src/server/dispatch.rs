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

use alloc::vec::Vec;

use nonos_nox_meter::caller::Role;
use nonos_payment_proto::{EINVAL, EPERM, OP_ACCOUNT, OP_DRAIN, OP_HEALTHCHECK, OP_LIST_TOKENS};
use nonos_payment_proto::{OP_CLAIM, OP_CLOSE, OP_JOB, OP_OPEN, OP_PAY, OP_SET_BUDGET, OP_TOPUP};
use nonos_payment_proto::{OP_LISTING_CAP, OP_ORDER_PLAINTEXT, OP_ORDER_REFUND, OP_ORDER_STATUS};
use nonos_payment_proto::{OP_ORDER_DELIVERED, OP_ORDER_NEW, OP_ORDER_OFFER, OP_PURCHASE};

use super::call::Call;
use super::gate::admits;
use crate::handlers;
use crate::jobs::Work;
use crate::meter::Meter;

/// Each op with the role its caller must play. The caller is
/// checked before the body is read. Op 7 (NXL1) is not in v1, and the
/// ORDER_* ops arrive with escrow (B5); until then, EINVAL.
pub fn dispatch(meter: &mut Meter, work: &mut Work, req: &Call<'_>) -> Vec<u8> {
    let role = match req.op {
        OP_HEALTHCHECK | OP_LIST_TOKENS => Role::Anyone,
        // JOB answers only the pid that started the job (Jobs::take).
        OP_PAY | OP_ORDER_STATUS | OP_JOB => Role::Payer,
        OP_DRAIN => Role::Settle,
        OP_ACCOUNT | OP_SET_BUDGET | OP_OPEN | OP_TOPUP | OP_CLOSE | OP_CLAIM => Role::Wallet,
        OP_ORDER_NEW | OP_ORDER_REFUND | OP_ORDER_OFFER | OP_LISTING_CAP => Role::Wallet,
        OP_ORDER_DELIVERED => Role::Deliverer,
        OP_ORDER_PLAINTEXT => Role::Install,
        // 19/0 the store's POST, 19/1 the wallet's TAKE.
        OP_PURCHASE => match handlers::purchase_role(req.body) {
            Some(r) => r,
            None => return req.answer(EINVAL, &[]),
        },
        _ => return req.answer(EINVAL, &[]),
    };
    if !admits(role, req.sender) {
        return req.answer(EPERM, &[]);
    }
    match req.op {
        OP_HEALTHCHECK => req.answer(0, &[]),
        OP_LIST_TOKENS => handlers::tokens(req),
        OP_PAY => handlers::pay(meter, req),
        OP_DRAIN => handlers::drain(meter, req),
        OP_ACCOUNT => handlers::account(meter, req),
        OP_OPEN => handlers::open(meter, work, req),
        OP_TOPUP => handlers::topup(meter, work, req),
        OP_CLOSE => handlers::close(meter, work, req),
        OP_JOB => handlers::job(work, req),
        OP_CLAIM => handlers::claim(meter, work, req),
        OP_ORDER_NEW => handlers::order_new(meter, work, req),
        OP_ORDER_STATUS => handlers::order_status(meter, work, req),
        OP_ORDER_REFUND => handlers::order_refund(req),
        OP_ORDER_OFFER => handlers::order_offer(meter, work, req),
        OP_ORDER_DELIVERED => handlers::order_delivered(meter, work, req),
        OP_ORDER_PLAINTEXT => handlers::order_plaintext(req),
        OP_PURCHASE => handlers::purchase(work, req),
        OP_LISTING_CAP => handlers::listing_cap(meter, req),
        _ => handlers::set_budget(meter, req),
    }
}
