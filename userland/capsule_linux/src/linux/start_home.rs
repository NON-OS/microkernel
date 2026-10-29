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

//! A program whose policy grants FILES_HOME starts in its home.
//!
//! Its home is /root, the family's own (file/private/home.rs). Started in /,
//! where it may not write, a program that names a file without a folder
//! (a wallet's "MyWallet.keys", a log beside it) was refused the write.

use super::guest::Guest;

/// Where the family's home is, as its programs see it.
const HOME: &[u8] = b"/root";

/// Open the family's home under `key` when its policy grants one, and make
/// it the guest's working directory.
pub(super) fn open(guest: &mut Guest, key: [u8; 32]) -> Result<(), &'static [u8]> {
    if !super::app::home() {
        return Ok(());
    }
    let policy = &guest.app.policy;
    super::file::choose_home(&policy.listing_id, &policy.program_blake3);
    if !super::file::open_home(key) {
        return Err(&b"[LINUX] its home could not be opened\n"[..]);
    }
    guest.cwd = HOME.to_vec();
    Ok(())
}
