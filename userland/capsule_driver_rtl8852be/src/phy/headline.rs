// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw89_phy_sel_headline` (Linux v6.12 rtw89 `phy.c`): a table's
//! leading headline entries (top nibble 0xf) name the RFE type and cut
//! each variant is for; the one for this board is chosen in rtw89's four
//! steps: exact match, this RFE with any cut, this RFE's highest cut, any
//! RFE's highest cut.

use super::table::{entries, target, DONT_CARE};

/// (headline count, chosen headline), or `None` when no variant fits.
pub fn headline(bin: &[u8], rfe: u8, cv: u8) -> Option<(usize, usize)> {
    let heads: alloc::vec::Vec<u32> =
        entries(bin).map(|e| e.0).take_while(|a| a >> 28 == 0xf).collect();
    if heads.is_empty() {
        return Some((0, 0));
    }
    let compare = |rfe: u32, cv: u32| rfe << 16 | cv;
    let exact = |c: u32| heads.iter().position(|&a| target(a) == c);
    if let Some(i) = exact(compare(rfe as u32, cv as u32)) {
        return Some((heads.len(), i));
    }
    if let Some(i) = exact(compare(rfe as u32, DONT_CARE)) {
        return Some((heads.len(), i));
    }
    // Highest cut for this RFE, then for any RFE; a tie takes the later one.
    for want in [rfe as u32, DONT_CARE] {
        let best = heads.iter().enumerate().filter(|(_, &a)| (a >> 16) & 0xff == want);
        if let Some((i, _)) = best.fold(None, |b: Option<(usize, u32)>, (i, &a)| match b {
            Some((_, cv)) if a & 0xff < cv => b,
            _ => Some((i, a & 0xff)),
        }) {
            return Some((heads.len(), i));
        }
    }
    None
}
