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

//! Moving a job on: an ask is sent and waited for; a sign is done here,
//! through the keyring, and moves on at once; done or failed ends the job.

use alloc::vec::Vec;

use nonos_nox_meter::flow::Next;
use nonos_payment_proto::EIO;

use super::ask::request;
use super::send::send;
use super::work::{Pending, Work};
use crate::handlers::deliver_key;
use crate::keyring::sign_tx;
use crate::orders::{bind, drop_order, keep, keep_offer};

enum Act {
    Sent(Pending),
    End(Result<Vec<u8>, i32>),
}

pub(super) fn advance(work: &mut Work, mut next: Result<Next, i32>, now_ms: u64) {
    loop {
        let Some(run) = work.run.as_mut() else { return };
        let act = match next {
            Err(e) => Act::End(Err(e)),
            Ok(Next::Done(hashes)) => Act::End(Ok(hashes)),
            Ok(Next::Keep(order, made)) => {
                Act::End(bind(&order, now_ms / 1000).and_then(|_| keep(&order)).map(|_| made))
            }
            Ok(Next::Drop(id, status)) => Act::End(drop_order(&id).map(|_| status)),
            Ok(Next::KeepOffer(o, made)) => Act::End(keep_offer(&o, now_ms / 1000).map(|_| made)),
            Ok(Next::Deliver(id, k)) => Act::End(deliver_key(&mut work.delivered, &id, &k)),
            Ok(Next::Sign(txs)) => {
                let raws: Result<Vec<Vec<u8>>, i32> =
                    txs.iter().map(|t| sign_tx(run.key_id, t)).collect();
                next = raws.map(|r| run.flow.on_signed(r));
                continue;
            }
            Ok(Next::Ask(ask)) => {
                work.next_req = work.next_req.wrapping_add(1);
                let req = work.next_req;
                match request(&ask, run.flow.work.chain_id(), &run.address) {
                    None => Act::End(Err(EIO)),
                    Some((op, body)) => match send(op, req, &body) {
                        Ok(settle_pid) => {
                            Act::Sent(Pending { op, req, settle_pid, sent_ms: now_ms })
                        }
                        Err(e) => Act::End(Err(e)),
                    },
                }
            }
        };
        match act {
            Act::Sent(p) => run.pending = Some(p),
            Act::End(r) => work.finish(r),
        }
        return;
    }
}
