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

//! The stop before reset, per version, against Linux rtl8169_cleanup and
//! rtl_wait_txrx_fifo_empty.

use nonos_libc::logged;

use super::model::window;
use crate::chip::MacVersion;
use crate::constants::regs::{REG_CMD, REG_RX_CONFIG, REG_TX_POLL};
use crate::hw::regs::{CMD_STOP_REQ, INTR_MITIGATE_RXTX_EMPTY, MCU_RXTX_EMPTY};
use crate::hw::regs::{REG_INTR_MITIGATE, REG_MCU};
use crate::hw::{quiesce, wait_txrx_fifo_empty};
use crate::regs::Regs;

fn said(text: &str) -> bool {
    logged().iter().any(|l| l.contains(text))
}

#[test]
fn quiescing_clears_only_the_accept_bits_of_rxconfig() {
    let bar = window();
    bar.present32(REG_RX_CONFIG, 0x0000_E70F);
    quiesce(&Regs::new(bar.base()), MacVersion(17));
    assert_eq!(bar.wrote32(REG_RX_CONFIG), 0x0000_E700);
    assert_ne!(bar.wrote8(REG_CMD) & CMD_STOP_REQ, 0, "StopReq on an 8168b");
}

#[test]
fn the_8111evl_requests_stop_and_waits_for_txcfg_empty() {
    let bar = window();
    quiesce(&Regs::new(bar.base()), MacVersion(34));
    assert_ne!(bar.wrote8(REG_CMD) & CMD_STOP_REQ, 0);
    assert!(said("(TxConfig TXCFG_EMPTY), resetting anyway"));
}

#[test]
fn the_8168dp_waits_for_its_normal_queue_to_go_idle() {
    let bar = window();
    bar.present8(REG_TX_POLL, 0x40);
    quiesce(&Regs::new(bar.base()), MacVersion(28));
    assert!(said("(TxPoll NPQ), resetting anyway"));
    assert_eq!(bar.wrote8(REG_CMD) & CMD_STOP_REQ, 0, "no StopReq on the 8168dp");
}

#[test]
fn the_8125a_waits_on_mcu_and_the_8125b_also_stops_and_waits_on_intrmitigate() {
    let bar = window();
    let regs = Regs::new(bar.base());
    bar.present8(REG_MCU, MCU_RXTX_EMPTY);
    assert_eq!(wait_txrx_fifo_empty(&regs, MacVersion(61)), Ok(()));
    assert_eq!(bar.wrote8(REG_CMD) & CMD_STOP_REQ, 0, "no StopReq on the 8125A");
    assert_eq!(wait_txrx_fifo_empty(&regs, MacVersion(63)), Err("IntrMitigate 0x0103"));
    assert_ne!(bar.wrote8(REG_CMD) & CMD_STOP_REQ, 0, "StopReq on the 8125B");
    bar.present16(REG_INTR_MITIGATE, INTR_MITIGATE_RXTX_EMPTY);
    assert_eq!(wait_txrx_fifo_empty(&regs, MacVersion(63)), Ok(()));
    bar.present8(REG_MCU, 0);
    assert_eq!(wait_txrx_fifo_empty(&regs, MacVersion(64)), Err("MCU RXTX_EMPTY"));
}
