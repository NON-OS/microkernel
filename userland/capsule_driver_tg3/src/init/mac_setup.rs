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

//! The station address in all four MAC address slots, the backoff seed
//! taken from it, the largest frame, the inter-packet gaps, and the receive
//! rules' default class (__tg3_set_mac_addr, then tg3_reset_hw).

use crate::constants::blocks::RCVLPC_CONFIG;
use crate::constants::literals::*;
use crate::constants::mac::*;
use crate::constants::MAC_LEN;
use crate::regs::Regs;

/// MAC_ADDR_0 to _3; only the 5703 and 5704 have the extended sixteen.
const ADDRESS_SLOTS: usize = 4;
const ADDRESS_STRIDE: usize = 8;

/// __tg3_set_one_mac_addr's (high, low) words.
pub fn address_words(mac: &[u8; MAC_LEN]) -> (u32, u32) {
    let high = (mac[0] as u32) << 8 | mac[1] as u32;
    let low = u32::from_be_bytes([mac[2], mac[3], mac[4], mac[5]]);
    (high, low)
}

/// The byte sum of the address, cut to the seed's ten bits.
pub fn backoff_seed(mac: &[u8; MAC_LEN]) -> u32 {
    mac.iter().map(|b| *b as u32).sum::<u32>() & TX_BACKOFF_SEED_MASK
}

pub fn tx_lengths() -> u32 {
    TX_LENGTHS_IPG_CRS << TX_LENGTHS_IPG_CRS_SHIFT
        | TX_LENGTHS_IPG << TX_LENGTHS_IPG_SHIFT
        | TX_LENGTHS_SLOT_TIME << TX_LENGTHS_SLOT_TIME_SHIFT
}

/*
 * SAFETY: `regs` carries the broker MmioMap base for BAR0; the address
 * slots, MAC_TX_BACKOFF_SEED, MAC_RX_MTU_SIZE, MAC_TX_LENGTHS,
 * MAC_RCV_RULE_CFG and RCVLPC_CONFIG are 32-bit registers inside it.
 */
pub fn program(regs: &Regs, mac: &[u8; MAC_LEN]) {
    let (high, low) = address_words(mac);
    unsafe {
        for n in 0..ADDRESS_SLOTS {
            regs.w32(MAC_ADDR_0_HIGH + n * ADDRESS_STRIDE, high);
            regs.w32(MAC_ADDR_0_LOW + n * ADDRESS_STRIDE, low);
        }
        regs.w32(MAC_TX_BACKOFF_SEED, backoff_seed(mac));
        regs.w32(MAC_RX_MTU_SIZE, RX_MTU_SIZE);
        regs.w32(MAC_TX_LENGTHS, tx_lengths());
        regs.w32(MAC_RCV_RULE_CFG, RCV_RULE_CFG_DEFAULT_CLASS);
        regs.w32(RCVLPC_CONFIG, RCVLPC_CONFIG_VALUE);
    }
}
