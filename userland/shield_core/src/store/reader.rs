//! A strict cursor over a decoded row. A short read is an error, never zero filled.
//! `done` refuses trailing bytes, so a row cannot smuggle a field this version does not know.

use crate::error::StoreError;

/// A cursor over one row's payload.
pub struct Reader<'a> {
    rest: &'a [u8],
}

impl<'a> Reader<'a> {
    /// A cursor over one row's payload.
    pub fn new(bytes: &'a [u8]) -> Reader<'a> {
        Reader { rest: bytes }
    }

    /// The next byte, or `RowShape` when the row ended first.
    pub fn byte(&mut self) -> Result<u8, StoreError> {
        let (first, rest) = self.rest.split_first().ok_or(StoreError::RowShape)?;
        self.rest = rest;
        Ok(*first)
    }

    /// The next little endian word, or `RowShape` when fewer than eight bytes remain.
    pub fn word(&mut self) -> Result<u64, StoreError> {
        let (head, rest) = self.rest.split_at_checked(8).ok_or(StoreError::RowShape)?;
        let mut word = [0u8; 8];
        word.copy_from_slice(head);
        self.rest = rest;
        Ok(u64::from_le_bytes(word))
    }

    /// The next four words, how a commitment is stored.
    pub fn quad(&mut self) -> Result<[u64; 4], StoreError> {
        let mut out = [0u64; 4];
        for slot in out.iter_mut() {
            *slot = self.word()?;
        }
        Ok(out)
    }

    /// True when every byte of the row was consumed.
    pub fn done(&self) -> bool {
        self.rest.is_empty()
    }
}
