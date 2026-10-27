// NONOS Operating System (AGPL-3.0-or-later)

//! What a 22000-family part writes into the receive rings and buffers.

use super::super::{Mem, Out};
use crate::gen3::plan::RX_USED;
use crate::gen3::region::Region;

/// Used-ring entry `i` naming buffer `vid`: gen2's four bytes, the BZ
/// one's four, or `iwl_rx_completion_desc` (rbid at 4).
pub(crate) fn put_used(ctrl: &Mem, i: usize, vid: u16, gen2: bool, v2: bool) {
    let (size, at) = if gen2 || v2 { (4, 0) } else { (32, 4) };
    let mut cd = vec![0u8; size];
    cd[at..at + 2].copy_from_slice(&vid.to_le_bytes());
    if gen2 {
        // Bits 12 to 31 are not the id: the part may leave anything there.
        cd[1] |= 0xF0;
        cd[2..4].copy_from_slice(&[0xA5, 0x5A]);
    }
    ctrl.write(RX_USED + i * size, &cd);
}

/// The packets for one buffer, each on a 64-byte boundary in gen2 mode.
pub(crate) fn packets(outs: &[Out], gen2: bool) -> Vec<u8> {
    let mut rb = Vec::new();
    for out in outs {
        let mut payload = out.payload.clone();
        if gen2 && out.cmd == 0xC1 && out.group == 0 && payload.len() >= 64 {
            // The version 1 descriptor: common 20, phy data 8, the rate at
            // 28, the energies to the TSF at 32..48, the frame at 48.
            let mut v1 = payload[..20].to_vec();
            v1.extend_from_slice(&[0; 8]);
            v1.extend_from_slice(&payload[36..40]);
            v1.extend_from_slice(&payload[40..56]);
            v1.extend_from_slice(&payload[64..]);
            payload = v1;
        }
        rb.resize(rb.len().next_multiple_of(64), 0);
        rb.extend_from_slice(&((4 + payload.len()) as u32).to_le_bytes());
        rb.extend_from_slice(&[out.cmd, out.group]);
        rb.extend_from_slice(&out.seq.to_le_bytes());
        rb.extend_from_slice(&payload);
    }
    if gen2 && outs.len() > 1 {
        rb.resize(rb.len().next_multiple_of(64), 0);
        rb.extend_from_slice(&0x5555_0000u32.to_le_bytes());
    }
    rb
}
