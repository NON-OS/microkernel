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

//! Building the inner plaintext of a record, and reading back what was decided.

use crate::handshake_step::{step, Step};

/// The inner plaintext of a record: the content, then its true type, then any
pub(crate) fn plain(content: &[u8], inner: u8, padding: usize) -> Vec<u8> {
    let mut out = content.to_vec();
    out.push(inner);
    out.resize(out.len() + padding, 0);
    out
}

pub(crate) fn decided(plain: &[u8]) -> (Option<Vec<u8>>, Option<u8>) {
    match step(plain) {
        Step::Messages(inner) => (Some(inner.to_vec()), None),
        Step::Stop(description) => (None, Some(description)),
        Step::Ignore => (None, None),
    }
}
