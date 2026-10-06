//! Reading a row's payload. Each branch requires the payload to end where the record does,
//! and an unknown kind is an error, so a newer store is never half read.

use super::status::from_byte;
use super::{Row, ACTIVITY, CURSOR, DEPOSIT, FOUND, STATUS};
use crate::error::StoreError;
use crate::notes::{NotePlaintext, NoteRecord};
use crate::store::activity::{from_quad, Activity};
use crate::store::reader::Reader;

/// Read one row payload.
pub fn decode(bytes: &[u8]) -> Result<Row, StoreError> {
    let mut r = Reader::new(bytes);
    let row = match r.byte()? {
        FOUND => {
            let value = r.word()?;
            let asset_id = r.word()?;
            let blinding = r.quad()?;
            let spend_pk = r.quad()?;
            let leaf_index = r.word()?;
            let cm = r.quad()?;
            let found_at = r.word()?;
            let status = from_byte(r.byte()?)?;
            Row::Found(NoteRecord {
                plain: NotePlaintext { value, asset_id, blinding, spend_pk },
                leaf_index,
                cm,
                status,
                found_at,
            })
        }
        STATUS => {
            let cm = r.quad()?;
            Row::Status { cm, status: from_byte(r.byte()?)? }
        }
        CURSOR => Row::Cursor(r.word()?),
        DEPOSIT => {
            let value = r.word()?;
            let asset_id = r.word()?;
            let blinding = r.quad()?;
            let spend_pk = r.quad()?;
            Row::Deposit(NotePlaintext { value, asset_id, blinding, spend_pk })
        }
        ACTIVITY => {
            let kind = r.byte()?;
            if !Activity::known(kind) {
                return Err(StoreError::RowKind);
            }
            let asset_id = r.word()?;
            let value = r.word()?;
            let at = r.word()?;
            let tx = from_quad(&r.quad()?);
            let tag = from_quad(&r.quad()?);
            let cm = r.quad()?;
            Row::Activity(Activity { kind, asset_id, value, at, tx, tag, cm })
        }
        _ => return Err(StoreError::RowKind),
    };
    if !r.done() {
        return Err(StoreError::RowShape);
    }
    Ok(row)
}
