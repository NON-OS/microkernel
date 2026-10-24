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

//! The only way the wallet puts text on the clipboard. Every app can read the
//! clipboard, so it takes an address and nothing else: never a key, words, a
//! hash or a sentence. Anything that is not an address is refused here.

mod rule;
pub use rule::copyable;

use nonos_app_skeleton::clients::clipboard::clipboard_copy;

pub const NOT_AN_ADDRESS: &[u8] = b"only an address is ever copied";
pub const NOT_TAKEN: &[u8] = b"the clipboard did not take it";

/// Copies `text` when it is one whole address, else says why not.
pub fn copy_address(text: &str) -> Result<(), &'static [u8]> {
    if !copyable(text) {
        return Err(NOT_AN_ADDRESS);
    }
    clipboard_copy(text.as_bytes()).map_err(|_| NOT_TAKEN)
}
