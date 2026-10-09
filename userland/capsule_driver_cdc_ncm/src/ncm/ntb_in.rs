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

//! The datagrams of one received NTB16, found as Linux cdc_ncm_rx_fixup
//! finds them: each NDP16 along the wNextNdpIndex chain, each entry up to
//! the first zero one, each datagram inside the transfer. What is malformed
//! ends what is taken from that NDP or from the block; nothing panics.

use alloc::collections::VecDeque;

use nonos_usbnet::nic::ETH_HEADER;

use super::ntb_out::NDP16_NOCRC_SIGN;
use super::verify::{le16, le32, verify_ndp16, verify_nth16, NDP16_HEADER};

/// cdc_ncm_rx_fixup's loopcount: the most NDPs read from one block.
const MAX_NDPS: usize = 50;

/// Queue each datagram of the NTB16 in `raw` as its (index, length).
pub fn datagrams(raw: &[u8], rx_max: usize, out: &mut VecDeque<(usize, usize)>) {
    let Some(mut at) = verify_nth16(raw, rx_max) else { return };
    let mut seen = [0usize; MAX_NDPS];
    for i in 0..MAX_NDPS {
        // An NDP read already means wNextNdpIndex loops back; its datagrams
        // are not taken twice.
        if seen[..i].contains(&at) {
            return;
        }
        seen[i] = at;
        let Some(n) = verify_ndp16(raw, at) else { return };
        // Only "NCM0": CRC mode is off, and Linux drops any other NDP.
        if le32(raw, at) == NDP16_NOCRC_SIGN {
            entries(raw, at, n, rx_max, out);
        }
        at = le16(raw, at + 6);
        if at == 0 {
            return;
        }
    }
}

fn entries(raw: &[u8], at: usize, n: usize, rx_max: usize, out: &mut VecDeque<(usize, usize)>) {
    for x in 0..n {
        let e = at + NDP16_HEADER + 4 * x;
        let (index, len) = (le16(raw, e), le16(raw, e + 2));
        // NCM 1.0, 3.7: entries after the first null one are ignored; Linux
        // ignores the rest after a bad one too.
        let inside = index <= raw.len() && len <= raw.len() - index;
        if index == 0 || len == 0 || !inside || len > rx_max || len < ETH_HEADER {
            return;
        }
        out.push_back((index, len));
    }
}
