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
//! A refund's whole step machine against a fake settle port: the capsule's
//! own step(), SettleCall and heard(), with the answers the test chooses.

use nonos_payment_proto::OrderMade;
use nonos_settle_proto::{reply, FLAG_ASYNC};

use super::dispute_fixture::ESCROW;
use super::fixture::{buy, made, offer, NOW, REFUND_PAIR, REFUND_TO};
use super::refund::SALT;
use crate::order::drive::escrow_step::step;
use crate::order::escrowing::{Ctx, Escrowing};
use crate::order::send_step::SendStep;
use crate::wallet::meter::order_calls::REFUND;
use crate::wallet::settle::send::{last, SENT};
use crate::wallet::settle::SettleCall;

pub(super) const CHAIN: u64 = 11_155_111;

pub(super) struct Rig {
    pub settle: SettleCall,
    pub e: Escrowing,
    pub now_ms: u64,
    m: OrderMade,
}

impl Rig {
    pub fn new(kind: u8) -> Rig {
        REFUND.with(|r| r.set(Some((REFUND_TO, SALT))));
        SENT.with(|s| s.borrow_mut().clear());
        let m = made(&buy().order_new(&offer(), REFUND_PAIR, NOW));
        Rig { settle: SettleCall::default(), e: Escrowing::new(kind), now_ms: 1_000, m }
    }

    /// One tick of the window: the deadline, then the step.
    pub fn tick(&mut self) -> bool {
        let timed = self.settle.tick(self.now_ms);
        let c = Ctx {
            chain_id: CHAIN,
            escrow: Some(ESCROW),
            made: &self.m,
            delivered: None,
            refund_to: REFUND_TO,
            pending: 0,
            now_ms: self.now_ms,
            now_s: NOW,
        };
        step(&mut self.settle, &mut self.e, &c) | timed
    }

    /// A frame from `pid` answering the last request, its id moved by `skew`.
    pub fn answer(&mut self, pid: u32, skew: u32, body: &[u8]) -> bool {
        let (op, id, _) = last().expect("nothing was sent");
        self.settle.frame(pid, &reply(op, FLAG_ASYNC, id.wrapping_add(skew), 0, body).unwrap())
    }

    pub fn step(&self) -> SendStep {
        self.e.step.clone()
    }
}
