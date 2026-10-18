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

//! The four transmit slots against what the part reports: a slot comes back
//! on TOK, TUN or TABT (an abort is cleared through TCR) and on nothing
//! else; four slots held answer "again", until the watchdog, past its bound
//! with the link up, resets the part and the send goes out.

use nonos_libc::{entropy, Deadline};

use super::serve::{ask, serve, status};
use crate::constants::dma::{TX_SLOT_BYTES, TX_SLOT_COUNT};
use crate::constants::regs::*;
use crate::init::{finish, TCR};
use crate::part::{driver_over, part};
use crate::protocol::{E_AGAIN, OP_TX_PACKET};

fn sends(n: usize) -> Vec<(u32, Vec<u8>)> {
    (0..n).map(|_| ask(OP_TX_PACKET, &[1; 60])).collect()
}

#[test]
fn a_slot_comes_back_on_ok_underrun_or_abort_and_on_nothing_else() {
    let _part = part(false);
    let mut pool = vec![0u8; TX_SLOT_COUNT * TX_SLOT_BYTES];
    let mut d = driver_over(0);
    d.tx_user_va = pool.as_mut_ptr() as u64;
    serve(&mut d, &sends(TX_SLOT_COUNT));
    let reported = [TX_STATUS_OK, TX_STATUS_UNDERRUN, TX_STATUS_ABORT, 0x3FFF];
    for (slot, tsd) in reported.into_iter().enumerate() {
        d.pio.w32(REG_TXSTATUS0 + slot as u16 * 4, tsd).expect("status written");
    }
    let got = serve(&mut d, &sends(1));
    assert_eq!(status(&got[0].1), 0);
    assert_eq!((d.tx_dirty, d.tx_cur), (3, 5), "three back, the fourth still the part's");
    assert_eq!(d.pio.r32(REG_TCR), Ok(TCR | TCR_CLEAR_ABORT), "the abort was cleared");
}

#[test]
fn four_slots_held_answer_again_until_the_watchdog_resets_a_stalled_part() {
    let _turn = entropy(true);
    let _part = part(false);
    let mut pool = vec![0u8; TX_SLOT_COUNT * TX_SLOT_BYTES];
    let mut d = finish(driver_over(0)).expect("brought up");
    d.tx_user_va = pool.as_mut_ptr() as u64;
    let codes: Vec<i32> = serve(&mut d, &sends(5)).iter().map(|r| status(&r.1)).collect();
    assert_eq!(codes, [0, 0, 0, 0, E_AGAIN], "four in flight, none finished, link up");
    d.tx_watch = Deadline::after_ms(0);
    let got = serve(&mut d, &sends(1));
    assert_eq!(status(&got[0].1), 0, "the reset made room and the frame went");
    assert_eq!((d.tx_dirty, d.tx_cur), (0, 1));
}
