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
//! A retry after a lost answer (v1.4.55): orderOf first, then the raw
//! already signed, sent again unchanged, or nothing once the call landed.

use nonos_settle_proto::{OP_BROADCAST, OP_INTENT};

use super::drive_path::{order_of, sent, to_broadcast, RAW};
use super::drive_rig::Rig;
use crate::order::send_step::{SendStep, REFUND};
use crate::settle_heard::STEP_MS;
use crate::wallet::settle::send::SETTLE_PID;

#[test]
fn a_lost_broadcast_goes_again_as_the_same_raw() {
    let mut r = Rig::new(REFUND);
    to_broadcast(&mut r);
    r.now_ms += STEP_MS;
    assert!(r.tick() && matches!(r.step(), SendStep::Failed(_)), "the answer was lost");
    let quotes = sent(OP_INTENT).len();
    r.e.again();
    r.tick();
    assert!(r.answer(SETTLE_PID, 0, &order_of(3)) && r.tick());
    assert_eq!(r.step(), SendStep::Broadcasting(RAW.to_vec()));
    r.tick();
    let raws = sent(OP_BROADCAST);
    assert_eq!((raws.len(), &raws[0]), (2, &raws[1]), "the same bytes twice");
    assert_eq!(sent(OP_INTENT).len(), quotes, "never quoted or signed again");
}

#[test]
fn a_retry_after_the_refund_landed_sends_nothing() {
    let mut r = Rig::new(REFUND);
    to_broadcast(&mut r);
    r.now_ms += STEP_MS;
    r.tick();
    r.e.again();
    r.tick();
    assert!(r.answer(SETTLE_PID, 0, &order_of(5)) && r.tick());
    assert_eq!((r.step(), sent(OP_BROADCAST).len()), (SendStep::Done, 1));
}
