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

//! What one decrypted record contributes to the handshake.

/// Inner content type for a handshake message, RFC 8446 section 5.
const HANDSHAKE: u8 = 22;

pub(super) enum Step<'a> {
    Messages(&'a [u8]),
    Stop(u8),
    Ignore,
}

pub(super) fn step(plain: &[u8]) -> Step<'_> {
    let Some((content, inner)) = super::inner_plain::split(plain) else {
        return Step::Ignore;
    };
    if inner == HANDSHAKE {
        return Step::Messages(content);
    }
    match super::alert::description_in_plaintext(inner, content) {
        Some(description) => Step::Stop(description),
        None => Step::Ignore,
    }
}
