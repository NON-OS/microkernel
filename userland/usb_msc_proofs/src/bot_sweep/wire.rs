// NONOS Operating System (AGPL-3.0-or-later)
//! The bytes on the bulk pipes: a CSW (BOT 1.0, 5.2, as recalled,
//! unverified), a data phase in pages, and the next answer the host reads.

use super::build::{Ans, PAGE};

pub fn csw(sig: &[u8; 4], tag: u32, residue: u32, status: u8) -> Vec<u8> {
    [&sig[..], &tag.to_le_bytes(), &residue.to_le_bytes(), &[status]].concat()
}

/// The data phase of `len` bytes, the device sending the first `d`.
pub fn pieces(data: &[u8], d: usize) -> Vec<Ans> {
    let (mut out, mut at) = (vec![], 0);
    while at < data.len() {
        let n = PAGE.min(data.len() - at);
        let take = n.min(d - at);
        out.push(if take == 0 { Ans::Zlp } else { Ans::Data(data[at..at + take].to_vec()) });
        at += take;
        if take < n {
            break;
        }
    }
    out
}

/// The next answer, counted as used even when there is none.
pub fn take<'a>(a: &'a [Ans], used: &mut usize) -> Option<&'a Ans> {
    *used += 1;
    a.get(*used - 1)
}
