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

//! One listed word: its length byte, its spelling, and the option it names.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgsError {
    /// More than MAX_LEN bytes.
    Long,
    /// No magic, or nothing after it.
    Magic,
    /// No words, or more than MAX_WORDS.
    Count,
    /// The bytes end inside a word, or a word of length zero.
    Short,
    /// A word that is not `--name` or `--name=value`.
    Word,
    /// Two words naming the same option.
    Twice,
    /// Bytes after the last word.
    Trailing,
}

/// The word at the front of `rest` and what follows it.
pub(super) fn read(rest: &[u8]) -> Result<(&[u8], &[u8]), ArgsError> {
    let (len, rest) = rest.split_first().ok_or(ArgsError::Short)?;
    let len = usize::from(*len);
    if len == 0 || rest.len() < len {
        return Err(ArgsError::Short);
    }
    let (word, after) = rest.split_at(len);
    match spelled(word) {
        true => Ok((word, after)),
        false => Err(ArgsError::Word),
    }
}

fn spelled(word: &[u8]) -> bool {
    let Some(body) = word.strip_prefix(b"--") else { return false };
    let n = name(word).len() - 2;
    let first = body.first().is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit());
    let named =
        body[..n].iter().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'-');
    let value = match body.get(n..) {
        Some([]) => true,
        Some([b'=', v @ ..]) => !v.is_empty() && v.iter().all(|c| (0x21..=0x7e).contains(c)),
        _ => false,
    };
    first && named && value
}

/// The option a word names: `--name`, without any `=value`.
pub fn name(word: &[u8]) -> &[u8] {
    word.iter().position(|c| *c == b'=').map_or(word, |at| &word[..at])
}

/// Whether the person's word `given` names the option of the listed `word`:
/// it is `--name` itself, or `--name=` and a value.
pub fn names(given: &[u8], word: &[u8]) -> bool {
    let n = name(word);
    given.strip_prefix(n).is_some_and(|rest| rest.is_empty() || rest[0] == b'=')
}
