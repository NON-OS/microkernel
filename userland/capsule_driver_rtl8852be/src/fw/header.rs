// NONOS Operating System (AGPL-3.0-or-later)

//! A firmware suit's header, format 0: `rtw89_fw_hdr_parser_v0` and
//! `rtw89_fw_update_ver_v0` (Linux v6.12 rtw89 `fw.c`, `struct
//! rtw89_fw_hdr` in `fw.h`). Eight words, then sixteen bytes per section,
//! then the optional dynamic header; the sections' bytes follow in order
//! and, with any security signatures, must end exactly at the suit's end.

/// `FWDL_SECTION_MAX_NUM`'s worth of room; a suit with more is refused.
pub const MAX_SECTIONS: usize = 10;
const SECURITY_TYPE: u32 = 9;
const SIG_LEN: usize = 512;
const CHECKSUM_LEN: usize = 8;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Section {
    pub dladdr: u32,
    pub offset: usize,
    pub len: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header {
    pub version: [u8; 4], // major, minor, sub, index
    /// The header as sent: the suit's head less any dynamic header.
    pub sent_len: usize,
    pub sections: [Section; MAX_SECTIONS],
    pub count: usize,
}

fn word(b: &[u8], i: usize) -> Option<u32> {
    let w = b.get(i * 4..i * 4 + 4)?;
    Some(u32::from_le_bytes([w[0], w[1], w[2], w[3]]))
}

pub fn parse(suit: &[u8]) -> Option<Header> {
    let w1 = word(suit, 1)?;
    let (w3, w6, w7) = (word(suit, 3)?, word(suit, 6)?, word(suit, 7)?);
    if w3 >> 24 != 0 {
        return None; // not format 0
    }
    let count = ((w6 >> 8) & 0xff) as usize;
    if count == 0 || count > MAX_SECTIONS {
        return None;
    }
    let base = 32 + 16 * count;
    let dynamic = w7 & (1 << 16) != 0;
    let hdr_len = if dynamic { ((w3 >> 16) & 0xff) as usize } else { base };
    if hdr_len < base {
        return None;
    }
    if dynamic && word(suit, base / 4)? as usize != hdr_len - base {
        return None; // the dynamic header's own length must agree
    }
    let mut sections = [Section::default(); MAX_SECTIONS];
    let (mut at, mut sigs) = (hdr_len, 0usize);
    for (i, s) in sections.iter_mut().take(count).enumerate() {
        let (s0, s1, s2) =
            (word(suit, 8 + i * 4)?, word(suit, 9 + i * 4)?, word(suit, 10 + i * 4)?);
        let mut len = (s1 & 0xff_ffff) as usize;
        if s1 & (1 << 28) != 0 {
            len += CHECKSUM_LEN;
        }
        if (s1 >> 24) & 0xf == SECURITY_TYPE {
            sigs = sigs.checked_add((s2 as usize).checked_mul(SIG_LEN)?)?;
        }
        *s = Section { dladdr: s0 & 0x1fff_ffff, offset: at, len };
        at = at.checked_add(len)?;
    }
    if at.checked_add(sigs)? != suit.len() {
        return None;
    }
    let version = w1.to_le_bytes();
    Some(Header { version, sent_len: base, sections, count })
}
