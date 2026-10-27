// NONOS Operating System (AGPL-3.0-or-later)

//! The multi-firmware container: `rtw89_mfw_recognize` (Linux v6.12 rtw89
//! `fw.c`, `struct rtw89_mfw_hdr` in `fw.h`). A 16-byte header (0xff, the
//! entry count) then 16-byte entries: cut, type, MP flag, the suit's offset
//! and size. rtw89 takes, of the wanted type and not MP, the entry with the
//! highest cut at or below the chip's; a file without the 0xff signature is
//! one legacy suit of the normal type.

pub const MFW_SIG: u8 = 0xff;
/// `enum rtw89_fw_type`.
pub const TYPE_NORMAL: u8 = 1;
pub const TYPE_WOWLAN: u8 = 3;
pub const TYPE_NORMAL_CE: u8 = 5;
const HDR: usize = 16;
const ENTRY: usize = 16;

/// One suit's place in the file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Suit {
    pub cv: u8,
    pub kind: u8,
    pub offset: usize,
    pub size: usize,
}

/// The entry `i` of the container, checked to lie inside `file`.
pub fn entry(file: &[u8], i: usize) -> Option<Suit> {
    let e = file.get(HDR + i * ENTRY..HDR + (i + 1) * ENTRY)?;
    let word = |at: usize| u32::from_le_bytes([e[at], e[at + 1], e[at + 2], e[at + 3]]) as usize;
    let (offset, size) = (word(4), word(8));
    let end = offset.checked_add(size)?;
    (e[2] == 0 && size > 0 && end <= file.len()).then_some(Suit {
        cv: e[0],
        kind: e[1],
        offset,
        size,
    })
}

/// The suit of `kind` for a chip of cut `cv`.
pub fn select(file: &[u8], kind: u8, cv: u8) -> Option<Suit> {
    match file.first() {
        Some(&MFW_SIG) => {}
        Some(_) if kind == TYPE_NORMAL => {
            return Some(Suit { cv, kind, offset: 0, size: file.len() });
        }
        _ => return None,
    }
    let n = *file.get(1)? as usize;
    (0..n).filter_map(|i| entry(file, i)).filter(|s| s.kind == kind && s.cv <= cv).fold(
        None,
        |best: Option<Suit>, s| match best {
            Some(b) if b.cv >= s.cv => Some(b),
            _ => Some(s),
        },
    )
}

/// `rtw89_fw_recognize` with the 8852B's `try_ce_fw`: the CE suit first,
/// the normal one when the file has none.
pub fn select_normal(file: &[u8], cv: u8) -> Option<Suit> {
    select(file, TYPE_NORMAL_CE, cv).or_else(|| select(file, TYPE_NORMAL, cv))
}
