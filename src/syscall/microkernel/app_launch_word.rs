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

//! The one word a served launch may give its run, pure. Held in kernel_proofs.
//!
//! SimpleX's core makes the person's profile on its first start only from
//! `--user-display-name=<name>`: with none it reads stdin, which a run has
//! closed, and exits before it listens. The window that launches it is what
//! knows the name, so MkAppLaunch carries one word, from a capsule SERVES
//! pairs with the listing alone, beginning with the prefix WORDS names for
//! that capsule: the window sets that one option, as plain text, and nothing
//! else. The run hands it to the program as one argument, after its listed.

/// The longest word, its prefix counted.
pub(crate) const MOST: usize = 96;

/// A table of capsules and what each is given: LISTINGS, or WORDS.
type Pairs<'t> = &'t [(&'t str, &'t str)];

/// The word a launch of `listing` carries: None for none; for a capsule
/// `owns` answers for that `tables.0` (LISTINGS) pairs with `listing`, a
/// word that begins with its prefix in `tables.1` (WORDS) and names more,
/// in at most MOST bytes of UTF-8 with no control byte. Err otherwise.
pub(crate) fn admit_word<'a>(
    owns: impl Fn(&str) -> bool,
    listing: &str,
    word: &'a [u8],
    tables: (Pairs<'_>, Pairs<'_>),
) -> Result<Option<&'a str>, ()> {
    if word.is_empty() {
        return Ok(None);
    }
    let mine = tables.0.iter().find(|&&(c, own)| owns(c) && own == listing);
    let prefix = mine.and_then(|&(c, _)| tables.1.iter().find(|&&(w, _)| w == c)).map(|r| r.1);
    let text = core::str::from_utf8(word).map_err(|_| ())?;
    let plain = !text.bytes().any(|b| b < 0x20 || b == 0x7f);
    let fits = |p: &str| !p.is_empty() && text.len() > p.len() && text.len() <= MOST;
    match prefix {
        Some(p) if fits(p) && plain && text.starts_with(p) => Ok(Some(text)),
        _ => Err(()),
    }
}
