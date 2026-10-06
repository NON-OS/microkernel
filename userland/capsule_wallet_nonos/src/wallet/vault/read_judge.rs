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

//! What a read of the vault store came to, judged from the answer alone.

use super::answer::Answer;

/// Why a read gave no blob.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Unread {
    /// The store did not answer in time, or refused the read: it says
    /// nothing about the file.
    Silent,
    /// The store answered with something that is not exactly a blob.
    NotABlob,
}

/// The blob in `rx`, whose data starts at `start`, or why there is none.
///
/// A short read is a refusal rather than a partial buffer. The blob is fixed
/// length by construction, so fewer bytes means a truncated file or a store
/// that answered something else, and either way there is nothing here to
/// hand to the opener. Padding to length would present a record that fails
/// its tag as though the wallet had been tampered with.
pub(super) fn judge<const N: usize>(
    answer: Answer,
    rx: &[u8],
    start: usize,
) -> Result<[u8; N], Unread> {
    let total = match answer {
        Answer::Ok(n) => n,
        /* A store that refused a read of a file it opened could not say
         * what the file holds: asked again, never taken for no file. */
        Answer::Refused(_) => return Err(Unread::Silent),
        Answer::Silent => return Err(Unread::Silent),
    };
    let data = rx.get(start..total).ok_or(Unread::NotABlob)?;
    if data.len() != N {
        return Err(Unread::NotABlob);
    }
    let mut out = [0u8; N];
    out.copy_from_slice(data);
    Ok(out)
}

/// Whether a record read back is the cleared one a keep writes in place of
/// the last (`remember::forget_vault`): every byte zero. The store replaces
/// a kept record only at its own length, so a cleared record keeps it. A
/// sealed record is the keyring's ciphertext and tag, never all zero.
pub(super) fn is_cleared(record: &[u8]) -> bool {
    record.iter().all(|b| *b == 0)
}
