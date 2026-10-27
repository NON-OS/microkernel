// NONOS Operating System (AGPL-3.0-or-later)

//! The physical cells to the logical map: `rtw89_dump_logical_efuse_map`
//! (Linux v6.12 rtw89 `efuse.c`). After the security-control bytes, each
//! record is two header bytes (a block index across them, and a word-enable
//! nibble whose clear bits mark words present) then two bytes per present
//! word; a header byte of 0xff ends the records. A record that would read
//! past the physical cells or write past the logical map is refused, as
//! rtw89 refuses it.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Corrupt;

/// Fill `log` (pre-filled 0xff by the caller) from `phy`, skipping
/// `sec_ctrl` bytes at either end.
pub fn logical(phy: &[u8], sec_ctrl: usize, log: &mut [u8]) -> Result<(), Corrupt> {
    let end = phy.len().checked_sub(sec_ctrl).ok_or(Corrupt)?;
    let mut at = sec_ctrl;
    while at < end {
        let (h1, h2) = (phy[at], *phy.get(at + 1).ok_or(Corrupt)?);
        if h1 == 0xff || h2 == 0xff {
            break;
        }
        let blk = ((h2 & 0xf0) >> 4) as usize | ((h1 & 0x0f) as usize) << 4;
        let word_en = h2 & 0xf;
        at += 2;
        for i in 0..4 {
            if word_en & (1 << i) != 0 {
                continue;
            }
            let li = (blk << 3) + (i << 1);
            if at + 1 > end - 1 || li + 1 > log.len() - 1 {
                return Err(Corrupt);
            }
            log[li] = phy[at];
            log[li + 1] = phy[at + 1];
            at += 2;
        }
    }
    Ok(())
}
