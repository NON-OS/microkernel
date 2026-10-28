// NONOS Operating System (AGPL-3.0-or-later)

//! A parameter table's walk: `rtw89_phy_sel_headline` and
//! `rtw89_phy_init_reg` (Linux v6.12 rtw89 `phy.c`, `phy.h`). Leading
//! headline entries pick the variant for this board (`headline`).
//! Branch entries (IF 8, ELIF 9, ELSE 0xa, END 0xb, CHECK 4) then select
//! which runs of plain entries are written.

/// The data files hold little-endian (addr, data) pairs.
pub fn entries(bin: &[u8]) -> impl Iterator<Item = (u32, u32)> + '_ {
    bin.chunks_exact(8).map(|c| {
        let w = |i: usize| u32::from_le_bytes([c[i], c[i + 1], c[i + 2], c[i + 3]]);
        (w(0), w(4))
    })
}

pub const DONT_CARE: u32 = 0xff;
pub const fn target(a: u32) -> u32 {
    a & 0x0fff_ffff
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableFail {
    /// No headline fits this board ("invalid PHY package").
    NoPackage,
    /// An ELSE came with no branch taken ("failed to load CR"); rtw89 stops
    /// that table there.
    Unmatched,
}

use super::headline::headline;

/// Hand `write` every entry this board takes, in order.
pub fn walk(bin: &[u8], rfe: u8, cv: u8, mut write: impl FnMut(u32, u32)) -> Result<(), TableFail> {
    let (size, idx) = headline(bin, rfe, cv).ok_or(TableFail::NoPackage)?;
    let cfg = entries(bin).nth(idx).map(|e| target(e.0)).unwrap_or(0);
    let (mut matched, mut found, mut tgt) = (true, false, 0);
    for (a, d) in entries(bin).skip(size) {
        match a >> 28 {
            0x8 | 0x9 => tgt = target(a),
            0xa if !found => return Err(TableFail::Unmatched),
            0xa => matched = false,
            0xb => (matched, found) = (true, false),
            0x4 if found => matched = false,
            0x4 => (matched, found) = (tgt == cfg, tgt == cfg),
            _ if matched => write(a, d),
            _ => {}
        }
    }
    Ok(())
}
