//! Reading one pool log: a leaf index from a topic, the sealed blob from the data, and the record
//! a found note is kept as.

use crate::notes::{NotePlaintext, NoteRecord, NoteStatus};

pub(super) fn record(plain: NotePlaintext, leaf_index: u64, leaf: &[u8; 32]) -> NoteRecord {
    let mut le = *leaf;
    le.reverse();
    let mut cm = [0u64; 4];
    for (w, chunk) in cm.iter_mut().zip(le.chunks_exact(8)) {
        let mut b = [0u8; 8];
        b.copy_from_slice(chunk);
        *w = u64::from_le_bytes(b);
    }
    NoteRecord { plain, leaf_index, cm, status: NoteStatus::Unspent, found_at: leaf_index }
}

pub(super) fn leaf_of(topic: &[u8; 32]) -> u64 {
    let mut b = [0u8; 8];
    b.copy_from_slice(topic.get(24..32).unwrap_or(&[0; 8]));
    u64::from_be_bytes(b)
}

/// One ABI `bytes` from a log's data: an offset word, a length word, the bytes.
pub(super) fn abi_bytes(data: &[u8]) -> Option<Vec<u8>> {
    let mut len = [0u8; 8];
    len.copy_from_slice(data.get(56..64)?);
    let n = usize::try_from(u64::from_be_bytes(len)).ok()?;
    Some(data.get(64..64usize.checked_add(n)?)?.to_vec())
}
