// NONOS Operating System (AGPL-3.0-or-later)

//! CCMP-128 in the modelled chip, under its key table (`keys`). A frame
//! sent with `PROTECT_FRAME` is sealed under its entry's pairwise key at the
//! chip's own packet number, or not sent. A protected frame heard is opened
//! under the station's pairwise key, or the own entry's group key at its
//! index, and comes up as `mt7921_mac_fill_rx` reads it: cipher and index in
//! RXD1, packet number in group 1, no CCMP header or MIC; a failed MIC sets
//! `ICV_ERR` over the plaintext. A frame no key matches is dropped: what the
//! driver must not depend on (`Faults::pass_unkeyed` passes it up instead).

use nonos_wifi_core::dot11::ccmp::{decrypt, encrypt, pn_and_key};
use nonos_wifi_core::dot11::header::header_len;

use super::chip::Chip;

/// The own entry, which holds the group keys.
const OWN: u16 = 19;

/// What the chip makes of a frame it heard: up as it came, or opened (RXD1's
/// bits, the packet number, the frame).
pub enum Heard {
    Plain,
    Opened(u32, u64, Vec<u8>),
    Dropped,
}

impl Chip {
    pub(super) fn seal(&mut self, wcid: u32, frame: &[u8]) -> Option<Vec<u8>> {
        let e = self.keys.entries.get_mut(&(wcid as u16));
        let Some((tk, e)) = e.and_then(|e| Some((e.pairwise?, e))) else {
            self.keys.dropped.1 += 1;
            return None;
        };
        e.tx_pn += 1;
        encrypt(frame, e.tx_pn, &tk)
    }

    pub(super) fn open(&mut self, f: &[u8]) -> Heard {
        if f.len() < 24 || f[1] & 0x40 == 0 {
            return Heard::Plain;
        }
        let hdr = header_len(u16::from_le_bytes([f[0], f[1]]));
        let Some((pn, id)) = pn_and_key(f, hdr) else { return Heard::Plain };
        let (e, ta) = (&self.keys.entries, f[10..16].try_into().ok());
        let key = if f[4] & 1 != 0 {
            e.get(&OWN).and_then(|e| e.group[usize::from(id)])
        } else {
            let peer = e.iter().find(|(w, e)| **w != OWN && e.addr.is_some() && e.addr == ta);
            peer.and_then(|(_, e)| e.pairwise)
        };
        let Some(key) = key else {
            self.keys.dropped.0 += u32::from(!self.faults.pass_unkeyed);
            return if self.faults.pass_unkeyed { Heard::Plain } else { Heard::Dropped };
        };
        let (body, icv) = match decrypt(f, &key) {
            Some(b) => (b, 0),
            None => (unchecked(f, hdr, pn, &key), 1 << 25),
        };
        let mut out = f[..hdr].to_vec();
        out.extend_from_slice(&body);
        Heard::Opened(4 << 16 | u32::from(id) << 21 | 1 << 11 | icv, pn, out)
    }
}

/// The plaintext of `f` with its MIC unchecked: the keystream is what sealing
/// zeros at the same packet number gives.
fn unchecked(f: &[u8], hdr: usize, pn: u64, key: &[u8; 16]) -> Vec<u8> {
    let ct = &f[hdr + 8..f.len() - 8];
    let zeros = [&f[..hdr], &vec![0; ct.len()][..]].concat();
    let ks = encrypt(&zeros, pn, key).unwrap();
    ct.iter().zip(&ks[hdr + 8..]).map(|(c, k)| c ^ k).collect()
}
