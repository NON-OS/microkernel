// NONOS Operating System (AGPL-3.0-or-later)

//! A CMAC control table command body: the control word (MAC ID, write),
//! eight value words, eight mask words. rtw89's `SET_CMC_TBL_*` helpers
//! (Linux v6.12 rtw89 `fw.h`) write a field and set its bits in the mask
//! word eight further on, so only the fields named change in the
//! firmware's table.

use super::media::MAC_ID;
use crate::regs::field;

pub const CMC_TBL_LEN: usize = 68;

pub struct Tbl([u32; 17]);

impl Tbl {
    /// MAC ID and the "write" operation in the control word.
    pub fn new() -> Tbl {
        let mut t = Tbl([0; 17]);
        t.0[0] = field(0x7f, MAC_ID as u32) | 0x80;
        t
    }
    pub fn set(&mut self, w: usize, mask: u32, v: u32) -> &mut Self {
        self.0[w] = (self.0[w] & !mask) | field(mask, v);
        self.0[w + 8] |= mask;
        self
    }
    pub fn bytes(&self) -> [u8; CMC_TBL_LEN] {
        let mut b = [0u8; CMC_TBL_LEN];
        for (i, v) in self.0.iter().enumerate() {
            b[4 * i..4 * i + 4].copy_from_slice(&v.to_le_bytes());
        }
        b
    }
}
