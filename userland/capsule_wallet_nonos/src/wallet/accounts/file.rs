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

/*
 * The account file: which accounts of the phrase are in use and which one
 * is open, as six bytes. Pure, so the wallet proofs run it on the host.
 */

/* Account 0 and up to seven more: the keyring's share for one owner. */
pub const MAX_ACCOUNTS: u8 = 8;
pub const FILE_LEN: usize = 6;
const MAGIC: [u8; 3] = *b"NXA";
const VERSION: u8 = 1;

pub fn encode(count: u8, open: u8) -> [u8; FILE_LEN] {
    [MAGIC[0], MAGIC[1], MAGIC[2], VERSION, count, open]
}

/* The count and the open account, or nothing for a file that is not one. */
pub fn decode(bytes: &[u8; FILE_LEN]) -> Option<(u8, u8)> {
    let (count, open) = (bytes[4], bytes[5]);
    let ok = bytes[..3] == MAGIC
        && bytes[3] == VERSION
        && (1..=MAX_ACCOUNTS).contains(&count)
        && open < count;
    ok.then_some((count, open))
}
