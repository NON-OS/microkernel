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

//! A market release's extra packages: Alpine packages its install adds to the
//! closure, each resolved through the signed index like any need. They ride
//! after the listed arguments, or after the policy when there are none, so
//! the publisher's signature covers them.
//!
//! `tail = [args block] [with block]`, `block = "NLXWITH1" | count u8 (1-8) |
//! count x (len u8 | name)`, each name `[a-z0-9][a-z0-9._+-]{0,63}`, none twice.

mod split;
use split::package;
pub use split::split;

pub const MAGIC: &[u8; 8] = b"NLXWITH1";
pub const MAX_NAMES: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WithError {
    /// No magic, or a count of 0 or past MAX_NAMES.
    Count,
    /// The bytes end inside a name, or one breaks the package rule.
    Name,
    /// A name twice, or bytes after the last.
    Trailing,
}

/// The number of names in a canonical block.
pub fn check(block: &[u8]) -> Result<usize, WithError> {
    let Some((&count, mut rest)) = block.strip_prefix(MAGIC).and_then(|b| b.split_first()) else {
        return Err(WithError::Count);
    };
    let (count, mut at) = (usize::from(count), 0);
    if count == 0 || count > MAX_NAMES {
        return Err(WithError::Count);
    }
    while let Some((len, after)) = rest.split_first().filter(|_| at < count) {
        let name = after.get(..usize::from(*len)).filter(|n| package(n)).ok_or(WithError::Name)?;
        if names(block).take(at).any(|before| before == name) {
            return Err(WithError::Trailing);
        }
        (at, rest) = (at + 1, &after[name.len()..]);
    }
    match (at == count, rest.is_empty()) {
        (true, true) => Ok(count),
        (false, _) => Err(WithError::Name),
        (true, false) => Err(WithError::Trailing),
    }
}

/// The names of a block `check` accepted.
pub fn names(block: &[u8]) -> impl Iterator<Item = &[u8]> {
    let mut rest = block.get(MAGIC.len() + 1..).unwrap_or_default();
    core::iter::from_fn(move || {
        let (len, after) = rest.split_first()?;
        let name = after.get(..usize::from(*len))?;
        rest = &after[name.len()..];
        Some(name)
    })
}
