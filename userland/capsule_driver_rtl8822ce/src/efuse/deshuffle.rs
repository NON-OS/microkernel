// NONOS Operating System (AGPL-3.0-or-later)

//! The logical map unpacked from the physical blocks, as rtw88
//! `rtw_dump_logical_efuse_map` (efuse.c) unpacks it: headers only in the
//! packed area (the physical size less the protected 124 bytes), and a block
//! that would run past the packed area or the 768-byte map refuses the whole
//! map, where the RTL8821CE's unpacker keeps what it had.

use super::layout::{LOG_SIZE, PACKED_SIZE, PHYS_SIZE};

/// Unpack `phys` into `log` (which starts all 0xFF); false when a block runs
/// past either map, as rtw88's -EINVAL.
pub(crate) fn deshuffle(phys: &[u8; PHYS_SIZE], log: &mut [u8; LOG_SIZE]) -> bool {
    let mut i = 0usize;
    while i < PACKED_SIZE {
        let (hdr1, hdr2) = (phys[i], phys[i + 1]);
        if hdr1 == 0xff || ((hdr1 & 0x1f) == 0x0f && hdr2 == 0xff) {
            break;
        }
        let (blk, word_en) = if (hdr1 & 0x1f) == 0x0f {
            i += 2;
            (((hdr2 & 0xf0) >> 1) | ((hdr1 >> 5) & 0x07), hdr2 & 0x0f)
        } else {
            i += 1;
            ((hdr1 & 0xf0) >> 4, hdr1 & 0x0f)
        };
        for w in 0..4 {
            if word_en & (1 << w) != 0 {
                continue;
            }
            let at = ((blk as usize) << 3) + (w << 1);
            if i + 1 > PACKED_SIZE || at + 1 > LOG_SIZE {
                return false;
            }
            log[at] = phys[i];
            log[at + 1] = phys[i + 1];
            i += 2;
        }
    }
    true
}
