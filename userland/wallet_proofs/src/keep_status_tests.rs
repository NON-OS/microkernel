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

//! What the status says of a kept wallet: the way back, a live session, the footer.

use crate::keep_plan::{kept_status, Kept};
use crate::wallet::vault::Unsealed;

#[test]
fn the_status_names_the_way_back_for_each_kind() {
    let lost = [
        Kept::Live,
        Kept::LastStays(Unsealed::Store),
        Kept::LastStays(Unsealed::DiskFull),
        Kept::Lost(Unsealed::DiskFull),
        Kept::Lost(Unsealed::NoMachineKey),
        Kept::Lost(Unsealed::Keyring),
        Kept::Lost(Unsealed::Store),
    ];
    for kept in lost {
        let words = kept_status(kept, false).unwrap();
        let key = kept_status(kept, true).unwrap();
        assert!(words.ends_with(b"write down the phrase"));
        assert!(key.ends_with(b"keep the private key"));
    }
}

#[test]
fn a_live_session_says_plainly_that_power_off_ends_the_wallet() {
    for from_key in [false, true] {
        let said = kept_status(Kept::Live, from_key).unwrap();
        assert!(said.starts_with(b"this is a live session: nothing is kept past power off"));
    }
}

#[test]
fn the_footer_reads_only_while_a_refresh_runs() {
    use crate::status_words::{kept, reading};
    assert_eq!(reading(false, true, false), None, "no wallet, nothing to read");
    assert_eq!(reading(true, true, false), Some("reading"));
    assert_eq!(reading(true, true, true), Some("reading"));
    assert_eq!(reading(true, false, true), Some("synced"));
    assert_eq!(
        reading(true, false, false),
        Some("not read"),
        "a refresh that ended unread is said"
    );
    assert_eq!(kept(false, false), None);
    assert_eq!(kept(true, true), Some("sealed"));
    assert_eq!(kept(true, false), Some("RAM only"));
}
