// NONOS Operating System (AGPL-3.0-or-later)

//! The chip's key table by WTBL entry, as the firmware is taken to keep it:
//! a record's basic or WTBL generic TLV names the entry's peer; a WTBL reset
//! takes its keys; `STA_REC_KEY_V2` sets CCMP as a station entry's pairwise
//! key or the own entry's (MUAR 0xe) group key at its index, BIP_CMAC as the
//! IGTK. A key TLV not its count's length, or another cipher, sets nothing.

use std::collections::HashMap;

#[derive(Default)]
pub struct Entry {
    pub addr: Option<[u8; 6]>,
    pub pairwise: Option<[u8; 16]>,
    pub group: [Option<[u8; 16]>; 4],
    pub igtk: Option<[u8; 16]>,
    /// The packet number the chip last sealed under.
    pub tx_pn: u64,
}

#[derive(Default)]
pub struct Keys {
    pub entries: HashMap<u16, Entry>,
    /// Key commands taken; frames dropped for want of a key, heard and sent.
    pub installs: u32,
    pub dropped: (u32, u32),
}

fn addr(t: &[u8], at: usize) -> Option<[u8; 6]> {
    t.get(at..at + 6).map(|a| a.try_into().unwrap())
}

fn le16(b: &[u8], at: usize) -> usize {
    b.get(at..at + 2).map_or(0, |w| usize::from(u16::from_le_bytes([w[0], w[1]])))
}

impl Keys {
    /// A `STA_REC_UPDATE` body; false if its key TLV is refused.
    pub fn sta_rec(&mut self, body: &[u8]) -> bool {
        let wcid = u16::from(body[1]) | u16::from(body[6]) << 8;
        let (own, mut at) = (body[5] == 0xe, 8);
        for _ in 0..le16(body, 2) {
            let (tag, len) = (le16(body, at), le16(body, at + 2));
            let Some(t) = body.get(at..at + len.max(4)) else { return false };
            let e = self.entries.entry(wcid).or_default();
            match tag {
                0 => e.addr = addr(t, 12),
                13 if t.get(5) == Some(&1) => *e = Entry { addr: addr(t, 16), ..Entry::default() },
                0x11 => return self.key(wcid, own, t),
                _ => {}
            }
            at += len.max(4);
        }
        true
    }

    fn key(&mut self, wcid: u16, own: bool, t: &[u8]) -> bool {
        let n = usize::from(t[5]);
        if t.len() != 8 + 36 * n || t[4] != 0 || n > 2 {
            return false;
        }
        let e = self.entries.entry(wcid).or_default();
        for k in t[8..].chunks(36) {
            let key: [u8; 16] = k[4..20].try_into().unwrap();
            match (k[0], k[1], k[3], own) {
                (5, 36, 16, false) => e.pairwise = Some(key),
                (5, 36, 16, true) => e.group[usize::from(k[2] & 3)] = Some(key),
                (10, 36, 16, true) => e.igtk = Some(key),
                _ => return false,
            }
        }
        self.installs += 1;
        true
    }
}
