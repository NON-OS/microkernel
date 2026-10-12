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
//! The step machine end to end (v1.4.55): each answer moves one step; an
//! answer from another pid, with a wrong id or op, or after STEP_MS moves
//! none.

use nonos_settle_proto::{OP_BROADCAST, OP_INTENT, OP_READ};

use super::drive_path::{order_of, to_broadcast, RAW};
use super::drive_rig::{Rig, CHAIN};
use crate::order::send_step::{SendStep, REFUND};
use crate::settle_heard::STEP_MS;
use crate::wallet::settle::send::{last, DOWN, SETTLE_PID};

#[test]
fn a_refund_walks_from_order_of_to_done() {
    let mut r = Rig::new(REFUND);
    to_broadcast(&mut r);
    let body = [&CHAIN.to_le_bytes()[..], &RAW].concat();
    assert_eq!(last().map(|f| (f.0, f.2)), Some((OP_BROADCAST, body)));
    assert!(r.answer(SETTLE_PID, 0, &[0xab; 32]) && r.tick());
    assert_eq!(r.step(), SendStep::Sent([0xab; 32]));
    r.now_ms += 10_000;
    r.tick();
    assert_eq!(last().map(|f| f.0), Some(OP_READ));
    assert!(r.answer(SETTLE_PID, 0, &order_of(5)) && r.tick());
    assert_eq!(r.step(), SendStep::Done);
}

#[test]
fn a_wrong_pid_id_or_op_or_a_late_answer_moves_nothing() {
    let mut r = Rig::new(REFUND);
    DOWN.with(|d| d.set(true));
    assert!(!r.tick() && last().is_none(), "settle not running: nothing asked");
    DOWN.with(|d| d.set(false));
    r.now_ms += 10_000;
    r.tick();
    assert!(!r.answer(99, 0, &order_of(3)), "another pid");
    assert!(!r.answer(0, 0, &order_of(3)), "no sender");
    assert!(!r.answer(SETTLE_PID, 1, &order_of(3)), "a wrong id");
    let (_, id, _) = last().unwrap();
    let other_op = nonos_settle_proto::reply(OP_INTENT, 1, id, 0, &order_of(3)).unwrap();
    assert!(!r.settle.frame(SETTLE_PID, &other_op), "a wrong op");
    assert!(!r.tick() && r.step() == SendStep::Reading, "nothing moved");
    r.now_ms += STEP_MS;
    assert!(r.tick() && matches!(r.step(), SendStep::Failed(_)), "ETIME ends it");
    assert!(!r.answer(SETTLE_PID, 0, &order_of(3)), "late: dropped");
}
