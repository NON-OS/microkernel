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

//! Starting what an installed package provides.

use crate::syscall::{call_raw, N_MK_APP_LAUNCH};

/// Start the program the listing's package installed.
pub fn mk_app_launch(listing: &[u8]) -> i64 {
    mk_app_launch_with(listing, &[])
}

/// The same, giving its run one word, empty for none: a capsule SERVES
/// pairs with the listing may, within the prefix it is given (the kernel's
/// app_launch_word.rs); the program has it as its last argument.
pub fn mk_app_launch_with(listing: &[u8], word: &[u8]) -> i64 {
    let (l, w) = (listing.as_ptr() as u64, word.as_ptr() as u64);
    call_raw(N_MK_APP_LAUNCH, [l, listing.len() as u64, w, word.len() as u64, 0, 0])
}
