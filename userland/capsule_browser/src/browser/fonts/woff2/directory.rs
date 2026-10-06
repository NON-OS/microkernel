// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use alloc::vec::Vec;
use core::ops::Range;

use super::cursor::Cursor;

/// The tags of flag values 0..=62 (WOFF2 5.2, Known Table Tags).
const KNOWN: &[u8; 252] = b"cmapheadhheahmtxmaxpnameOS/2postcvt fpgmglyflocaprepCFF VORGEBDT\
EBLCgasphdmxkernLTSHPCLTVDMXvheavmtxBASEGDEFGPOSGSUBEBSCJSTFMATHCBDTCBLCCOLRCPALSVG sbixacnt\
avarbdatblocbslncvarfdscfeatfmtxfvargvarhstyjustlcarmortmorxopbdproptrakZapfSilfGlatGlocFeatSill";

/// One table of the directory: its tag, whether its bytes in the
/// decompressed stream are transformed, its sfnt length, and where its
/// bytes sit in the stream.
pub(super) struct Table {
    pub(super) tag: [u8; 4],
    pub(super) transformed: bool,
    pub(super) orig_len: usize,
    pub(super) src: Range<usize>,
}

/// The table directory (WOFF2 5.2). glyf and loca carry the transform
/// at version 0 and none otherwise; any other table carries none at
/// version 0 and one of hmtx's at 1. A transformed loca takes no bytes.
pub(super) fn read_directory(c: &mut Cursor, n: usize) -> Option<Vec<Table>> {
    let mut tables: Vec<Table> = Vec::new();
    tables.try_reserve_exact(n).ok()?;
    let mut at = 0usize;
    for _ in 0..n {
        let flags = c.u8()?;
        let tag: [u8; 4] = match (flags & 0x3f) as usize {
            63 => c.u32()?.to_be_bytes(),
            k => KNOWN[4 * k..4 * k + 4].try_into().ok()?,
        };
        let version = flags >> 6;
        let transformed = match &tag {
            b"glyf" | b"loca" => version == 0,
            b"hmtx" if version == 1 => true,
            _ if version == 0 => false,
            _ => return None,
        };
        let orig_len = c.base128()? as usize;
        let len = if transformed { c.base128()? as usize } else { orig_len };
        if (&tag == b"loca" && transformed && len != 0) || tables.iter().any(|t| t.tag == tag) {
            return None;
        }
        tables.push(Table { tag, transformed, orig_len, src: at..at.checked_add(len)? });
        at += len;
    }
    Some(tables)
}

/// The table with `tag`, if the font has one.
pub(super) fn find<'t>(tables: &'t [Table], tag: &[u8; 4]) -> Option<&'t Table> {
    tables.iter().find(|t| &t.tag == tag)
}
