// NONOS Operating System (AGPL-3.0-or-later)

//! The logical efuse map unpacked from the physical blocks.

use super::layout::EFUSE_SIZE;

// Unpack the physical efuse into the logical map. The physical efuse is a run of
// blocks: a one- or two-byte header names the block and which of its four words
// are present, then those words follow. A word-enable bit set means the word was
// never written and is skipped, leaving the 0xff the caller pre-filled.
pub(crate) fn deshuffle(phys: &[u8; EFUSE_SIZE], log: &mut [u8; EFUSE_SIZE]) {
    let mut i = 0usize;
    while i + 1 < EFUSE_SIZE {
        let hdr1 = phys[i];
        let hdr2 = phys[i + 1];
        // An erased header (or an erased second byte of a two-byte header) ends
        // the map.
        if hdr1 == 0xff || ((hdr1 & 0x1f) == 0x0f && hdr2 == 0xff) {
            break;
        }

        let blk_idx;
        let word_en;
        if (hdr1 & 0x1f) == 0x0f {
            blk_idx = ((((hdr2 & 0xf0) >> 1) | ((hdr1 >> 5) & 0x07)) as usize) << 3;
            word_en = hdr2 & 0x0f;
            i += 2;
        } else {
            blk_idx = (((hdr1 & 0xf0) >> 4) as usize) << 3;
            word_en = hdr1 & 0x0f;
            i += 1;
        }

        for w in 0..4 {
            if word_en & (1 << w) != 0 {
                continue;
            }
            let log_idx = blk_idx + (w << 1);
            if i + 1 >= EFUSE_SIZE || log_idx + 1 >= EFUSE_SIZE {
                return;
            }
            log[log_idx] = phys[i];
            log[log_idx + 1] = phys[i + 1];
            i += 2;
        }
    }
}
