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
 * The account file the wallet keeps beside its vault: what it writes reads
 * back the same, and nothing else reads as a list of accounts, so a damaged
 * file opens account 0 alone rather than a wrong account.
 */

use crate::accounts_file::{decode, encode, FILE_LEN, MAX_ACCOUNTS};

#[test]
fn every_list_the_wallet_can_hold_reads_back() {
    for count in 1..=MAX_ACCOUNTS {
        for open in 0..count {
            assert_eq!(decode(&encode(count, open)), Some((count, open)));
        }
    }
}

#[test]
fn an_open_account_outside_the_list_is_refused() {
    for count in 1..=MAX_ACCOUNTS {
        assert_eq!(decode(&encode(count, count)), None);
    }
}

#[test]
fn no_accounts_or_too_many_is_refused() {
    assert_eq!(decode(&encode(0, 0)), None);
    assert_eq!(decode(&encode(MAX_ACCOUNTS + 1, 0)), None);
    assert_eq!(decode(&encode(255, 3)), None);
}

#[test]
fn any_other_bytes_are_not_a_list() {
    let good = encode(3, 1);
    for at in 0..4 {
        let mut bad = good;
        bad[at] ^= 0x20;
        assert_eq!(decode(&bad), None, "byte {at} changed");
    }
    assert_eq!(decode(&[0u8; FILE_LEN]), None);
}

#[test]
fn eight_accounts_fill_one_owners_keyring_share() {
    // Each account holds its key and its words in the keyring store, whose
    // per-owner share is read from the keyring's own constants.
    assert_eq!(MAX_ACCOUNTS as usize * 2, crate::store::MAX_KEYS_PER_OWNER);
}
