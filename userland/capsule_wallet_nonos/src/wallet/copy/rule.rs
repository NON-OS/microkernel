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

//! What may go on the clipboard: one whole address, public or private. Pure,
//! so the wallet proofs run it on the host.

use crate::wallet::screen::shield::typed::address_ok;

/// A public address, `0x` and 40 hex digits in any case, or a whole
/// private `nox1` address. Nothing around it, not even a space.
pub fn copyable(text: &str) -> bool {
    public(text) || address_ok(text)
}

fn public(text: &str) -> bool {
    match text.strip_prefix("0x") {
        Some(hex) => hex.len() == 40 && hex.bytes().all(|b| b.is_ascii_hexdigit()),
        None => false,
    }
}
