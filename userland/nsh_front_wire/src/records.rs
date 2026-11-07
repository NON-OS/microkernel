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

//! `OP_RECORDS`'s answer: the machine's own records, as nsh.attest read
//! them from its store for one connection. Four fields, each a u32 length
//! and its bytes, in this order: the `nsh-enrol/1` certificate, the signed
//! role table, the 32-byte key id the table is pinned under, and the
//! signed revocation list. An empty field is a record the machine does not
//! hold yet. Each field has a cap, and the whole answer stays under the
//! kernel's 1 MiB message; a frame with a field over its cap, a short
//! field or bytes after the last one does not split.

/// The enrolment certificate's cap: nsh/1 carries it with a u16 length.
pub const ENROL_MAX: usize = 0xFFFF;
/// The role table's cap: eight authorities, sixteen approvers and sixteen
/// roles fit with room to spare.
pub const TABLE_MAX: usize = 128 * 1024;
/// The revocation list's cap: nsh_roles' 4096 revoked grants and both
/// signatures fit.
pub const LIST_MAX: usize = 512 * 1024;
/// The anchor's one length when it is held.
pub const ANCHOR_LEN: usize = 32;

const FIELDS: usize = 4;
const LEN_BYTES: usize = 4;

/// The four records, borrowed from a frame or from what was read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Records<'a> {
    pub enrol: &'a [u8],
    pub table: &'a [u8],
    pub anchor: &'a [u8],
    pub list: &'a [u8],
}

impl<'a> Records<'a> {
    fn fields(&self) -> [&'a [u8]; FIELDS] {
        [self.enrol, self.table, self.anchor, self.list]
    }

    /// Whether every field is within its cap, and the anchor is empty or
    /// exactly `ANCHOR_LEN` bytes.
    pub fn within_caps(&self) -> bool {
        self.enrol.len() <= ENROL_MAX
            && self.table.len() <= TABLE_MAX
            && (self.anchor.is_empty() || self.anchor.len() == ANCHOR_LEN)
            && self.list.len() <= LIST_MAX
    }

    /// Whether the machine holds every record a connection needs.
    pub fn complete(&self) -> bool {
        self.fields().iter().all(|f| !f.is_empty())
    }
}

/// The encoded length of `r`, or `None` when a field is over its cap.
pub fn records_len(r: &Records<'_>) -> Option<usize> {
    if !r.within_caps() {
        return None;
    }
    r.fields().iter().try_fold(0usize, |n, f| n.checked_add(LEN_BYTES)?.checked_add(f.len()))
}

/// Lay `r` into `out`, which must hold `records_len(r)` bytes; returns the
/// length written.
pub fn write_records(out: &mut [u8], r: &Records<'_>) -> Option<usize> {
    let total = records_len(r)?;
    let out = out.get_mut(..total)?;
    let mut at = 0usize;
    for f in r.fields() {
        let len = u32::try_from(f.len()).ok()?;
        let end = at.checked_add(LEN_BYTES)?;
        out.get_mut(at..end)?.copy_from_slice(&len.to_le_bytes());
        let stop = end.checked_add(f.len())?;
        out.get_mut(end..stop)?.copy_from_slice(f);
        at = stop;
    }
    Some(total)
}

/// Split an `OP_RECORDS` answer's body into its four records.
pub fn split_records(body: &[u8]) -> Option<Records<'_>> {
    let mut rest = body;
    let mut take = || -> Option<&[u8]> {
        let (len, tail) = rest.split_first_chunk::<LEN_BYTES>()?;
        let len = usize::try_from(u32::from_le_bytes(*len)).ok()?;
        if len > tail.len() {
            return None;
        }
        let (field, after) = tail.split_at(len);
        rest = after;
        Some(field)
    };
    let r = Records { enrol: take()?, table: take()?, anchor: take()?, list: take()? };
    (rest.is_empty() && r.within_caps()).then_some(r)
}
