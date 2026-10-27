// NONOS Operating System (AGPL-3.0-or-later)

//! The chip's keys as its firmware is taken to keep them: the security CAM
//! by entry (each `H2C_FUNC_MAC_SEC_UPD`, its type and key), and address
//! CAM entry 0's seven key slots (words 9 to 11 of each CAM update), each a
//! security CAM entry and a key index. What the chip does with them on the
//! air is `sec_air`'s.

use std::collections::HashMap;

#[derive(Default)]
pub struct Sec {
    pub entries: HashMap<u8, (u8, [u8; 16])>,
    /// Address CAM entry 0's slots: (security CAM entry, key index).
    pub slots: [Option<(u8, u8)>; 7],
}

fn w(b: &[u8], i: usize) -> u32 {
    b.get(4 * i..4 * i + 4).map_or(0, |x| u32::from_le_bytes(x.try_into().unwrap()))
}

impl Sec {
    pub fn sec_upd(&mut self, b: &[u8]) -> Result<(), String> {
        let (w0, w1) = (w(b, 0), w(b, 1));
        if b.len() != 24 || w0 >> 8 & 0xff != 0 || w0 >> 16 & 0xff != 20 || w1 & !0xf != 0 {
            return Err(format!("sec cam body {w0:#x} {w1:#x} of {}", b.len()));
        }
        self.entries.insert(w0 as u8, ((w1 & 0xf) as u8, b[8..24].try_into().unwrap()));
        Ok(())
    }

    pub fn cam(&mut self, b: &[u8]) {
        let (w9, w10, w11) = (w(b, 9), w(b, 10), w(b, 11));
        let ent = |s: usize| if s < 3 { w10 >> (8 + 8 * s) } else { w11 >> (8 * (s - 3)) } as u8;
        self.slots = core::array::from_fn(|s| {
            (w10 >> s & 1 != 0).then(|| (ent(s), (w9 >> (18 + 2 * s) & 3) as u8))
        });
    }

    pub(super) fn key(&self, slots: core::ops::Range<usize>, id: u8) -> Option<[u8; 16]> {
        let (e, _) = self.slots[slots].iter().flatten().find(|(_, k)| *k == id)?;
        self.entries.get(e).filter(|(t, _)| *t == 6).map(|(_, k)| *k)
    }
}
