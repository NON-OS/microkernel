//! A row's payload bytes: fields in declaration order, little endian, no padding.
//! The kind fixes the shape, so a layout change needs a new kind, never an edit to an old one.

use super::status::to_byte;
use super::{Row, ACTIVITY, CURSOR, DEPOSIT, FOUND, STATUS};
use crate::store::activity::to_quad;

fn words(out: &mut Vec<u8>, w: &[u64]) {
    for v in w {
        out.extend_from_slice(&v.to_le_bytes());
    }
}

impl Row {
    /// The row as a payload for sealing.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(96);
        match self {
            Row::Found(r) => {
                out.push(FOUND);
                words(&mut out, &[r.plain.value, r.plain.asset_id]);
                words(&mut out, &r.plain.blinding);
                words(&mut out, &r.plain.spend_pk);
                words(&mut out, &[r.leaf_index]);
                words(&mut out, &r.cm);
                words(&mut out, &[r.found_at]);
                out.push(to_byte(r.status));
            }
            Row::Status { cm, status } => {
                out.push(STATUS);
                words(&mut out, cm);
                out.push(to_byte(*status));
            }
            Row::Cursor(at) => {
                out.push(CURSOR);
                words(&mut out, &[*at]);
            }
            Row::Deposit(p) => {
                out.push(DEPOSIT);
                words(&mut out, &[p.value, p.asset_id]);
                words(&mut out, &p.blinding);
                words(&mut out, &p.spend_pk);
            }
            Row::Activity(a) => {
                out.push(ACTIVITY);
                out.push(a.kind);
                words(&mut out, &[a.asset_id, a.value, a.at]);
                words(&mut out, &to_quad(&a.tx));
                words(&mut out, &to_quad(&a.tag));
                words(&mut out, &a.cm);
            }
        }
        out
    }
}
