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

//! The mode words and MAC values, worked by hand from tg3.c and held to the
//! driver's: both DMA engines, the GPIOs on a LOM, what the receiver keeps,
//! the rules zeroed, the address words, backoff seed and gaps.

use crate::constants::chip::CHIPREV_ID_57765_A0;
use crate::constants::mac::*;
use crate::init::engines::MAC_MODE_ENABLES;
use crate::init::{filters, mac_setup, modes};

use super::build::REV_57766;

#[test]
fn dma_engine_modes() {
    // Error enables, long bursts, IPv4 and IPv6 LSO; 2K reads on the 57766.
    assert_eq!(modes::rdmac_mode(CHIPREV_ID_57765_A0), 0x1803_03FE);
    assert_eq!(modes::rdmac_mode(REV_57766), 0x1883_03FE);
    assert_eq!(modes::wdmac_mode(), 0x2000_03FE);
    assert_eq!(MAC_MODE_ENABLES, 0x00E0_4800);
}

#[test]
fn a_lom_keeps_the_bootcodes_gpios() {
    let word = 0x0100_0018;
    assert_eq!(modes::grc_local_ctrl(word, false, 0xFFFF_FFFF), word);
    // GPIO 0 and 2 as the bootcode left them, GPIO1 driven high.
    let kept = modes::grc_local_ctrl(word | 0x0001_4800, true, 0x0000_2000);
    assert_eq!(kept, word | 0x0000_3000 | 0x0000_8000);
}

#[test]
fn the_receiver_keeps_vlan_tags_unless_asf_runs() {
    let on = RX_MODE_ENABLE | RX_MODE_IPV6_CSUM_ENABLE | RX_MODE_FLOW_CTRL_ENABLE;
    assert_eq!(filters::rx_mode(on | RX_MODE_PROMISC, false), on | RX_MODE_KEEP_VLAN_TAG);
    assert_eq!(filters::rx_mode(on | RX_MODE_KEEP_VLAN_TAG, true), on);
    assert_eq!(filters::rules_zeroed(false), 4..8);
    assert!(filters::rules_zeroed(true).is_empty());
}

#[test]
fn address_words_seed_and_gaps() {
    let mac = [0x02, 0x11, 0x22, 0x33, 0x44, 0xFF];
    assert_eq!(mac_setup::address_words(&mac), (0x0211, 0x2233_44FF));
    assert_eq!(mac_setup::backoff_seed(&mac), (0x02 + 0x11 + 0x22 + 0x33 + 0x44 + 0xFF) & 0x3FF);
    assert_eq!(mac_setup::backoff_seed(&[0xFF; 6]), 0x5FA & 0x3FF);
    assert_eq!(mac_setup::tx_lengths(), 0x2620);
}
