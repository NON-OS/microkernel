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

//! What a new passphrase must be: text, at least `MIN_CHARS` characters and
//! at most `MAX_BYTES` bytes, not only spaces, and typed the same twice.

pub const MIN_CHARS: usize = 8;
pub const MAX_BYTES: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// Fewer than `MIN_CHARS` characters, or only spaces.
    Short,
    /// More than `MAX_BYTES` bytes.
    Long,
    /// Not UTF-8 text.
    NotText,
    /// The confirmation differs.
    Mismatch,
}

/// Whether `passphrase` may be chosen.
pub fn acceptable(passphrase: &[u8]) -> Result<(), Refusal> {
    if passphrase.len() > MAX_BYTES {
        return Err(Refusal::Long);
    }
    let text = core::str::from_utf8(passphrase).map_err(|_| Refusal::NotText)?;
    if text.chars().count() < MIN_CHARS || text.chars().all(char::is_whitespace) {
        return Err(Refusal::Short);
    }
    Ok(())
}

/// Whether `passphrase`, confirmed by `again`, may be chosen.
pub fn chosen(passphrase: &[u8], again: &[u8]) -> Result<(), Refusal> {
    acceptable(passphrase)?;
    if !crate::wipe::ct_eq(passphrase, again) {
        return Err(Refusal::Mismatch);
    }
    Ok(())
}

/// Whether `passphrase` may be offered for checking: anything a person can
/// type that fits the wire.
pub fn checkable(passphrase: &[u8]) -> bool {
    !passphrase.is_empty() && passphrase.len() <= MAX_BYTES
}
