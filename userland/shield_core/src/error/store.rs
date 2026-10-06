//! Failures on the note store path. A row that does not authenticate is a tamper or a wrong
//! key, a finding kept apart from ordinary file failures.

/// Failures on the note store path. The store is a sealed append only log, so a
/// row that does not authenticate is a tamper or a wrong key, never a warning.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoreError {
    /// The file could not be opened, read or extended.
    Io,
    /// A row's tag did not verify.
    RowAuth,
    /// A row's framing was not the shape the reader expects.
    RowShape,
    /// A row decoded, but its contents are not a record this version knows.
    RowKind,
    /// The nonce counter reached its end, and the store must be rekeyed.
    NonceExhausted,
    /// The requested note is not in the store, or is no longer spendable.
    NoteMissing,
}

impl core::fmt::Display for StoreError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            StoreError::Io => "the note store could not be read or written",
            StoreError::RowAuth => "a stored row did not authenticate",
            StoreError::RowShape => "a stored row is malformed",
            StoreError::RowKind => "a stored row is from a newer version",
            StoreError::NonceExhausted => "the note store needs rekeying",
            StoreError::NoteMissing => "that note is not spendable",
        })
    }
}

impl core::error::Error for StoreError {}
