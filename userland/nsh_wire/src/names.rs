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

//! The names a peer sends: a user name, a role, a host and a capability. Each
//! is plain ASCII of a fixed shape, so a name cannot hide a control byte,
//! a second spelling of the same name, or anything a terminal would act on.

use crate::WireError;

pub const MAX_USER: usize = 32;
pub const MAX_ROLE: usize = 16;
pub const MAX_HOST: usize = 253;
const MAX_LABEL: usize = 63;

fn shaped(ok: bool) -> Result<(), WireError> {
    ok.then_some(()).ok_or(WireError::Value)
}

fn starts_lower(name: &[u8]) -> bool {
    name.first().is_some_and(u8::is_ascii_lowercase)
}

/// A kernel capability's name: `[A-Z][A-Za-z0-9]*`, its length checked by
/// the reader.
pub fn cap(name: &[u8]) -> Result<(), WireError> {
    let first_upper = name.first().is_some_and(u8::is_ascii_uppercase);
    shaped(first_upper && name.iter().all(u8::is_ascii_alphanumeric))
}

/// `[a-z][a-z0-9._-]{0,31}`
pub fn user(name: &[u8]) -> Result<(), WireError> {
    let rest = |b: &u8| b.is_ascii_lowercase() || b.is_ascii_digit() || b"._-".contains(b);
    shaped(name.len() <= MAX_USER && starts_lower(name) && name.iter().all(rest))
}

/// `[a-z][a-z0-9-]{0,15}`
pub fn role(name: &[u8]) -> Result<(), WireError> {
    let rest = |b: &u8| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-';
    shaped(name.len() <= MAX_ROLE && starts_lower(name) && name.iter().all(rest))
}

/// A lowercase DNS name: labels of 1 to 63 of `[a-z0-9-]`, not starting or
/// ending with `-`, joined by single dots, 253 bytes at most, no final dot.
pub fn host(name: &[u8]) -> Result<(), WireError> {
    let label_ok = |l: &[u8]| {
        let inner = |b: &u8| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-';
        (1..=MAX_LABEL).contains(&l.len())
            && l.iter().all(inner)
            && l.first() != Some(&b'-')
            && l.last() != Some(&b'-')
    };
    shaped((1..=MAX_HOST).contains(&name.len()) && name.split(|b| *b == b'.').all(label_ok))
}
