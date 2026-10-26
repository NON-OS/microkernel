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

//! Which program of a package its install records, the one `run` starts and
//! a market release's policy is bound to. Pure, so the host proofs hold it
//! and tools/nonos_market_catalogue/apk.py, which writes the policy, follows
//! the same rule.
//!
//! With no policy (a shipped tier, a family package): `usr/bin/<name>` when
//! the package has an entry there, ELF or not, else its first ELF under
//! usr/bin, in archive order.
//!
//! With a market release's policy: the program whose bytes the policy names.
//! A file's bytes are its last copy, the one that lands. Of the names under
//! usr/bin whose last copy is an ELF of those bytes, `usr/bin/<name>` if it
//! is one, else the first in archive order. So a package of many programs
//! (monero's wallet among its daemon and tools) starts as the one its
//! listing sells, and a policy naming bytes the package does not carry
//! records nothing, which bind refuses.

pub const BIN: &[u8] = b"usr/bin/";
const ELF: &[u8] = b"\x7fELF";

/// The path to record, from `files` as (name, body) in archive order.
pub fn choose<'a>(
    files: &[(&'a [u8], &'a [u8])],
    name: &[u8],
    want: Option<&[u8; 32]>,
    hash: impl Fn(&[u8]) -> [u8; 32],
) -> Option<&'a [u8]> {
    let own =
        |n: &[u8]| n.len() == BIN.len() + name.len() && n.starts_with(BIN) && n.ends_with(name);
    let Some(want) = want else {
        if let Some(&(n, _)) = files.iter().find(|(n, _)| own(n)) {
            return Some(n);
        }
        return files.iter().find(|(n, b)| n.starts_with(BIN) && b.starts_with(ELF)).map(|f| f.0);
    };
    let last = |n: &[u8]| files.iter().rev().find(|(m, _)| *m == n).map(|f| f.1);
    let named = |n: &[u8]| {
        n.starts_with(BIN) && last(n).is_some_and(|b| b.starts_with(ELF) && hash(b) == *want)
    };
    if let Some(&(n, _)) = files.iter().find(|(n, _)| own(n) && named(n)) {
        return Some(n);
    }
    files.iter().find(|(n, _)| named(n)).map(|f| f.0)
}
