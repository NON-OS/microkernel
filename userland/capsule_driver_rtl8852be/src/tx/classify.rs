// NONOS Operating System (AGPL-3.0-or-later)

//! What `rtw89_core_tx_update_desc_info` (Linux v6.12 rtw89 `core.c`)
//! reads from a frame: management (or a null function) or data, the
//! sequence number, broadcast or multicast receiver, the header length
//! (`ieee80211_hdrlen`), and whether it is an EAPOL frame (mac80211 marks
//! those `IEEE80211_TX_CTRL_PORT_CTRL_PROTO`; here, an unprotected data
//! frame whose LLC/SNAP carries ethertype 0x888e).

use super::wd::Desc;

const LLC_EAPOL: [u8; 8] = [0xaa, 0xaa, 0x03, 0, 0, 0, 0x88, 0x8e];

/// `ieee80211_hdrlen` for a management or data frame; `None` for control
/// frames, which the station never sends.
pub fn hdr_len(fc: u16) -> Option<usize> {
    let (ftype, stype) = ((fc >> 2) & 0x3, (fc >> 4) & 0xf);
    match ftype {
        0 => Some(24),
        2 => {
            let four = fc & 0x0300 == 0x0300;
            let qos = stype & 0x8 != 0;
            let htc = qos && fc & 0x8000 != 0;
            Some(24 + if four { 6 } else { 0 } + if qos { 2 } else { 0 } + if htc { 4 } else { 0 })
        }
        _ => None,
    }
}

/// The frame's descriptor fields, rates left 0 for the caller; `None` for
/// a frame too short for its header, or a control frame.
pub fn classify(mpdu: &[u8]) -> Option<Desc> {
    let fc = u16::from_le_bytes([*mpdu.first()?, *mpdu.get(1)?]);
    let hl = hdr_len(fc)?;
    if mpdu.len() < hl || mpdu.len() > 0x3fff {
        return None;
    }
    let (ftype, stype) = ((fc >> 2) & 0x3, (fc >> 4) & 0xf);
    let nullfunc = ftype == 2 && (stype == 0x4 || stype == 0xc);
    let mgmt = ftype == 0 || nullfunc;
    let protected = fc & 0x4000 != 0;
    let eapol = !mgmt && !protected && mpdu.get(hl..hl + 8) == Some(&LLC_EAPOL[..]);
    Some(Desc {
        mgmt,
        pkt_size: mpdu.len() as u16,
        seq: u16::from_le_bytes([mpdu[22], mpdu[23]]) >> 4,
        is_bmc: mpdu[4] & 0x1 != 0,
        hdr_len: hl as u8,
        rate: 0,
        bk: eapol,
    })
}
