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

//! A market release's listed arguments: the long options its program starts
//! with, before the person's own. They ride after the 256 policy bytes of the
//! release's guest_policy, so the publisher's signature covers them.
//!
//! ```text
//! block = "NLXARG01" | count u8 (1-16) | count x (len u8 | word)
//! word  = "--" name [ "=" value ], name [a-z0-9][a-z0-9-]*,
//!         value printable ASCII with no space; no name twice
//! ```
//!
//! At most MAX_LEN bytes, nothing after the last word, and no shell reads
//! it: each word is one argv entry as written.

mod word;
pub use word::{name, names, ArgsError};

pub const MAGIC: &[u8; 8] = b"NLXARG01";
/// The most bytes a block holds, magic included.
pub const MAX_LEN: usize = 1024;
/// The most words one block holds.
pub const MAX_WORDS: usize = 16;
/// What a machine vouches for beside a program: this, its policy, the block.
pub const VOUCH_DOMAIN: &[u8] = b"NONOS.linux.args.v1";

/// How many words `block` holds, when it is canonical.
pub fn check(block: &[u8]) -> Result<usize, ArgsError> {
    if block.len() > MAX_LEN {
        return Err(ArgsError::Long);
    }
    let Some((count, mut rest)) = block.strip_prefix(MAGIC).and_then(|b| b.split_first()) else {
        return Err(ArgsError::Magic);
    };
    let count = usize::from(*count);
    if count == 0 || count > MAX_WORDS {
        return Err(ArgsError::Count);
    }
    for at in 0..count {
        let (this, after) = word::read(rest)?;
        if words(block).take(at).any(|before| name(before) == name(this)) {
            return Err(ArgsError::Twice);
        }
        rest = after;
    }
    match rest.is_empty() {
        true => Ok(count),
        false => Err(ArgsError::Trailing),
    }
}

/// The words of `block`, which `check` has accepted. Reading stops at the
/// first byte that is not a word, so an unchecked block yields a prefix.
pub fn words(block: &[u8]) -> impl Iterator<Item = &[u8]> {
    let mut rest = block.get(MAGIC.len() + 1..).unwrap_or_default();
    core::iter::from_fn(move || {
        let (this, after) = word::read(rest).ok()?;
        rest = after;
        Some(this)
    })
}
