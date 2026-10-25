// NONOS Operating System (AGPL-3.0-or-later)

//! What Settings kept, read whole or refused. The record is read one byte
//! past VALUES_MAX, and one that fills it is refused: its framing could
//! still decode a prefix, and a cut record would be applied.

use alloc::vec::Vec;

use crate::field::Field;
use crate::settings_record::{decode_values, Value, VALUES_MAX};

/// What to ask the read for.
pub const VALUES_ASK: usize = VALUES_MAX + 1;

/// The values of `raw`, read with [`VALUES_ASK`], when it is a whole record.
pub fn kept_values(raw: &[u8]) -> Option<Vec<(Field, Value<'_>)>> {
    if raw.len() > VALUES_MAX {
        return None;
    }
    decode_values(raw)
}
