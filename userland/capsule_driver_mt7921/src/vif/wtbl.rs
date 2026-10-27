/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The station record's WTBL request, `STA_REC_WTBL` holding a
//! `wtbl_req_hdr` (`WTBL_RESET_AND_SET`) and, when enabled, `wtbl_generic`,
//! `wtbl_rx` and `wtbl_hdr_trans` (Linux v6.12 `mt76_connac_mcu.h`), as
//! `mt76_connac_mcu_alloc_wtbl_req`, `mt76_connac_mcu_wtbl_generic_tlv`
//! and `mt76_connac_mcu_wtbl_hdr_trans_tlv` fill them for a station
//! interface: to-DS, no receive header translation.

use alloc::vec::Vec;

use super::sta::Peer;
use super::sta_tlv::tlv;

/// `STA_REC_WTBL`: its length grows with the WTBL request inside it.
/// `muar` is the interface's entry for a station, 0xe for its own.
pub fn wtbl(out: &mut Vec<u8>, p: &Peer, enable: bool, muar: u8) {
    let t = tlv(out, 13, 4);
    let w = p.wcid.to_le_bytes();
    out.extend_from_slice(&[w[0], 1, 0, 0, w[1], 0, 0, 0]); // WTBL_RESET_AND_SET
    if enable {
        let g = tlv(out, 0, 20);
        out[g + 4..g + 10].copy_from_slice(&p.addr);
        out[g + 10] = muar;
        out[g + 16..g + 18].copy_from_slice(&p.aid.to_le_bytes());
        let r = tlv(out, 1, 12);
        (out[r + 5], out[r + 6], out[r + 7]) = (1, 1, 1); // rca1, rca2, rv
        let h = tlv(out, 6, 8);
        (out[h + 4], out[h + 6]) = (1, 1); // to_ds, no_rx_trans
        out[t + 6..t + 8].copy_from_slice(&3u16.to_le_bytes()); // nested count
    }
    let len = (out.len() - t) as u16;
    out[t + 2..t + 4].copy_from_slice(&len.to_le_bytes());
}
