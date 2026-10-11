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
//! No double sends (v1.4.55): a retry re-sends the raw already signed and
//! never a new one, and a gas withdraw under way holds the button.

use super::fixture::REFUND_TO;
use crate::order::escrow_offer::Offer;
use crate::order::escrowing::Escrowing;
use crate::order::send_resume::{gas_offer, resume, GAS_WAIT_S};
use crate::order::send_step::{EscrowSend, SendStep, REFUND};

#[test]
fn a_retry_after_a_lost_broadcast_resends_the_same_raw() {
    let raw = vec![0x02, 0xf8, 0x71, 0x99];
    let quoting = || SendStep::Quoting(EscrowSend { kind: 11, from: REFUND_TO, calldata: vec![] });
    assert_eq!(resume(quoting(), Some(&raw), false), SendStep::Broadcasting(raw.clone()));
    assert_eq!(
        resume(SendStep::RefundTo((3, 0)), Some(&raw), false),
        SendStep::Broadcasting(raw.clone())
    );
    assert_eq!(resume(quoting(), Some(&raw), true), SendStep::Done, "it landed");
    assert_eq!(resume(SendStep::Failed("x"), Some(&raw), false), SendStep::Failed("x"));
    assert_eq!(resume(quoting(), None, false), quoting(), "nothing was signed");
}

#[test]
fn try_again_keeps_the_signed_raw_and_the_paid_gas() {
    let mut e = Escrowing::new(REFUND);
    (e.step, e.signing, e.next_ms) = (SendStep::Failed("lost"), true, 9);
    (e.raw, e.gas_paid_s) = (Some(vec![7; 3]), Some(100));
    e.again();
    assert_eq!((e.step.clone(), e.signing, e.next_ms), (SendStep::Reading, false, 0));
    assert_eq!((e.raw, e.gas_paid_s), (Some(vec![7; 3]), Some(100)));
}

#[test]
fn a_second_press_while_the_gas_withdraw_is_under_way_starts_nothing() {
    let pay = Some(Offer::PayGas);
    assert_eq!(gas_offer(pay, None, 50), pay, "not paid yet");
    assert_eq!(gas_offer(pay, Some(50), 50), Some(Offer::GasWaiting));
    assert_eq!(gas_offer(pay, Some(50), 50 + GAS_WAIT_S - 1), Some(Offer::GasWaiting));
    assert_eq!(gas_offer(pay, Some(50), 50 + GAS_WAIT_S), pay, "ten minutes on");
    assert_eq!(gas_offer(pay, Some(u64::MAX), 0), Some(Offer::GasWaiting), "no wrap");
    let again = Some(Offer::Again);
    assert_eq!(gas_offer(again, Some(50), 50), again, "only the gas is held");
}
